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
        for (_, slope) in actions.actions {
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
        self.actions.push((old_color, action.1), -slope);
        
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

        //self.delete(action);
        //self.add(action);
    }

    fn add(&mut self, action: Action) {
        todo!()
    }

    fn delete(&mut self, action: Action) {
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
}