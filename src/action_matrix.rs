use bit_fiddler::unset;
use bit_iter::BitIter;
use priority_queue::PriorityQueue;
pub use itertools::Itertools;

use crate::colored_graph::*;

pub type EdgePos = usize;
pub type Action = (Color, EdgePos);

pub struct ActionMatrix {
    counts: [[Iyy; E]; C],
    graph: ColoredGraph,
    actions: PriorityQueue<Action, Iyy>
}

impl From<ColoredGraph> for ActionMatrix {
    fn from(graph: ColoredGraph) -> Self {
        let mut counts: [[Iyy; E]; C] = [[0; E]; C];
        let mut actions: PriorityQueue<Action, Iyy> = Default::default();
        for (pos, (u, v)) in (0..N)
            .tuple_combinations().enumerate()
        {
            let old_color = graph.color((u, v)).unwrap();
            let old_count = graph.count_edge_cliques(old_color, (u,v));
            counts[old_color][pos] = old_count;
            for new_color in 0..C {
                if new_color != old_color {
                    let new_count = graph.count_edge_cliques(new_color, (u,v));
                    counts[new_color][pos] = new_count;
                    actions.push((new_color, pos), old_count - new_count);
                }
            }
        }
        ActionMatrix { counts, graph, actions }
    }
}

#[cfg(test)]
mod action_matrix_initialization {
    use super::*;

    #[test]
    fn correct_number_of_acounts() {
        let graph = ColoredGraph::red();
        let actions = ActionMatrix::from(graph);
        assert_eq!(actions.actions.len(), (C-1) * E)
    }

    #[test]
    fn red_graph_action_gradients() {
        let graph = ColoredGraph::red();
        let actions = ActionMatrix::from(graph);
        for ((color, pos), slope) in actions.actions {
            assert_ne!(color, 0);
            assert_eq!(slope, choose(N-2, S[0]-2) as Iyy);
        }
    }
}

impl ActionMatrix {
    pub fn slope(&self, action: Action) -> Option<&Iyy> {
        self.actions.get_priority(&action)
    }

    fn remove_slope(&mut self, action: Action) -> (Action, Iyy) {
        self.actions.remove(&action).unwrap()
    }
    
    pub fn recolor(&mut self, action: Action, old_color: Color) {
        let (_, slope) = self.remove_slope(action);
        self.actions.push(action, -slope);
        
        let (new_color, pos) = action;
        let column_change = 
            self.counts[new_color][pos] - 
            self.counts[old_color][pos];
        for c in 0..C {
            if c != old_color && c != new_color {
                let action = (c, pos);
                self.actions.change_priority_by(
                    &action, 
                    |slope| *slope += column_change
                );
            }
        }

        let edge = pos_to_edge(pos);
        self.delete(old_color, edge);
        //self.add(action);
    }

    fn delete(&mut self, old_color: Color, (u, v): Edge) {
        let s = S[old_color];
        if s < 3 { return }

        let neighbors_uv = self.graph.common_neighborhood(old_color, u, v);
        for (u, v) in [(u, v), (v, u)] {
            let neighbors_u = unset!((self.graph.bit_neighborhood(old_color, u)), Uxx, v);
            for w in BitIter::from(neighbors_u) {
                let neighbors_uvw = neighbors_uv & self.graph.bit_neighborhood(old_color, w);
                let count_uvw = self.graph.count_cliques(old_color, Some(s-3), Some(neighbors_uvw));
                self.decrement_count(old_color, (v,w), count_uvw);
            }
        }

        if s < 4 { return }
        todo!()
    }

    fn decrement_count(&mut self, color: Color, edge: Edge, amount: Iyy) {
        let pos = edge_to_pos(edge);
        self.counts[color][pos] -= amount;
        let curr_color = self.graph.color(edge).unwrap();
        if curr_color == color {
            for other_color in 0..C {
                if other_color != color {
                    self.actions.change_priority_by(
                        &(other_color, pos), 
                        |slope| *slope -= amount
                    );
                }
            }
        }
        else {
            self.actions.change_priority_by(
                &(color, pos),
                |slope| *slope += amount
            );
        }
    }
    
    fn add(&mut self, new_color: Color, (u, v): Edge) {
        todo!()
    }
}

#[cfg(test)]
mod recolor_gradient_test {
    use super::*;
    #[test]
    fn firsts_column_change() {
        let mut actions = ActionMatrix::from(ColoredGraph::red());
        actions.recolor((1, 0), 0);
        for c in 0..C {
            let slope = actions.slope((c, 0));
            match c {
                0 => assert_eq!(slope, Some(&-(choose(N-2, S[0]-2) as Iyy))),
                1 => assert_eq!(slope, None),
                _ => assert_eq!(slope, Some(&0))
            }
        }
    }

    #[test]
    fn deletion_change() {
        let mut actions = ActionMatrix::from(ColoredGraph::red());
        actions.recolor((1, 0), 0);
        for (i, (u,v)) in (0..N).tuple_combinations().enumerate() {
            match (u, v) {
                (0, 1) => assert_eq!(actions.slope((0, i)), None),
                (0, _) | (1, _) => assert_eq!(actions.slope((1, i)), Some(&(choose(N-3, S[0]-2) as Iyy))),
                (_, _) => assert_eq!(actions.slope((1, i)), Some(&(choose(N-2, S[0]-2) as Iyy))),
            }
        }
    }
}