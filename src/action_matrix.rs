use std::marker::PhantomData;

use crate::{prelude::*, neighborhood::*};

use priority_queue::PriorityQueue;
pub use itertools::Itertools;
use rand::rngs::ThreadRng;

use crate::colored_graph::*;

pub type EdgePos = usize;
pub type Action = (Color, EdgePos);

#[derive(Clone)]
pub struct ActionMatrix<T: Neighborhood, const C: usize, const N: usize, const E: usize> {
    counts: [[Iyy; E]; C],
    graph: ColoredGraph<T, C, N>,
    actions: PriorityQueue<Action, Iyy>,
    totals: [Iyy; C],
    phantom: PhantomData<T>
}

impl<T: Neighborhood, const C: usize, const N: usize, const E: usize> From<ColoredGraph<T, C, N>> for ActionMatrix<T, C, N, E> {
    fn from(graph: ColoredGraph<T, C, N>) -> Self {
        let mut counts: [[Iyy; E]; C] = [[0; E]; C];
        let mut actions: PriorityQueue<Action, Iyy> = Default::default();
        let mut totals: [Iyy; C] = [0; C];
        for (pos, (u, v)) in (0..N)
            .tuple_combinations().enumerate()
        {
            let old_color = graph.color((u, v)).unwrap();
            let old_count = graph.count_edge_cliques(old_color, (u,v));
            totals[old_color] += old_count;
            counts[old_color][pos] = old_count;
            for (new_color, color_counts) in counts.iter_mut().enumerate() {
                if new_color != old_color {
                    let new_count = graph.count_edge_cliques(new_color, (u,v));
                    color_counts[pos] = new_count;
                    actions.push((new_color, pos), old_count - new_count);
                }
            }
        }

        for c in 0..C {
            totals[c] /= choose(S[c], 2)
        }

        ActionMatrix { counts, graph, actions, totals, phantom: PhantomData }
    }
}

#[cfg(test)]
mod action_matrix_initialization {
    use super::*;

    const C: usize = 2;
    const N: usize = 8;
    const E: usize = choose_two(N);

    type T = UxxN<N>;

    #[test]
    fn correct_number_of_acounts() {
        let graph = ColoredGraph::<T, C, N>::red();
        let actions = ActionMatrix::<T, C, N, E>::from(graph);
        assert_eq!(actions.actions.len(), (C-1) * E)
    }

    #[test]
    fn red_graph_action_gradients() {
        let graph = ColoredGraph::<T, C, N>::red();
        let actions = ActionMatrix::<T, C, N, E>::from(graph);
        for ((color, _), slope) in actions.actions {
            assert_ne!(color, 0);
            assert_eq!(slope, choose(N-2, S[0]-2));
        }
    }
}

impl<T: Neighborhood, const C: usize, const N: usize, const E: usize>
ActionMatrix<T, C, N, E> {
    pub fn graph(&self) -> &ColoredGraph<T, C, N> { &self.graph }
    pub fn actions_mut(&mut self) -> &mut PriorityQueue<Action, Iyy> { &mut self.actions }
    
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

        let edge = pos_to_edge::<N>(pos);
        self.delete(old_color, edge);
        self.add(new_color, edge);

        self.totals[old_color] -= self.counts[old_color][pos];
        self.totals[new_color] += self.counts[new_color][pos];
    }

    fn delete(&mut self, old_color: Color, edge: Edge) {
        self.toggle::<true>(old_color, edge)
    }

    fn add(&mut self, new_color: Color, edge: Edge) {
        self.toggle::<false>(new_color, edge)
    }

    fn toggle<const IS_DELETION: bool>
    (&mut self, color: Color, (u, v): Edge)
    {
        let s = S[color];
        if s < 3 { return }

        let neighbors_uv = self.graph.common_neighborhood(color, u, v);
        for (u, v) in [(u, v), (v, u)] {
            let mut neighbors_u = self.graph.bit_neighborhood(color, u);
            neighbors_u.delete(v);
            for w in neighbors_u.iter() {
                let neighbors_uvw = neighbors_uv & self.graph.bit_neighborhood(color, w);
                let count_uvw = self.graph.count_cliques(color, Some(s-3), Some(neighbors_uvw));
                self.adjust_count::<IS_DELETION>(color, (v,w), count_uvw)
            }
        }

        if s < 4 { return }
        
        for (w, x) in neighbors_uv.iter().tuple_combinations() {
            let candidates = neighbors_uv & self.graph.common_neighborhood(color, w, x);
            let count_uvwx = self.graph.count_cliques(color, Some(s-4), Some(candidates));
            self.adjust_count::<IS_DELETION>(color, (w,x), count_uvwx)
        }
    }

    fn adjust_count<const IS_DELETION: bool>
    (&mut self, color: Color, edge: Edge, amount: Iyy) {
        if amount == 0 { return }
        if IS_DELETION { self.decrement_count(color, edge, amount) }
        else           { self.increment_count(color, edge, amount) }
    }

    fn decrement_count(&mut self, color: Color, edge: Edge, amount: Iyy) {
        let pos = edge_to_pos::<N>(edge);
        self.counts[color][pos] -= amount;
        let curr_color = self.graph.color(edge).unwrap();
        if curr_color == color {
            for other_color in 0..C {
                if other_color != color {
                    self.actions.change_priority_by(        // todo!("benchmark speed from .increase_priority")
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

    fn increment_count(&mut self, color: Color, edge: Edge, amount: Iyy) {
        let pos = edge_to_pos::<N>(edge);
        self.counts[color][pos] += amount;
        let curr_color = self.graph.color(edge).unwrap();
        if curr_color == color {
            for other_color in 0..C {
                if other_color != color {
                    self.actions.change_priority_by(
                        &(other_color, pos), 
                        |slope| *slope += amount
                    );
                }
            }
        }
        else {
            self.actions.change_priority_by(
                &(color, pos),
                |slope| *slope -= amount
            );
        }
    }
}

#[cfg(test)]
mod recolor_gradient_test {
    use super::*;

    impl<T: Neighborhood, const C: usize, const N: usize, const E: usize>
    ActionMatrix<T, C, N, E> {
        pub fn slope(&self, action: Action) -> Option<&Iyy> {
            self.actions.get_priority(&action)
        }
    }

    const C: usize = 2;
    const N: usize = 8;
    const E: usize = choose_two(N);

    type T = UxxN<N>;

    #[test]
    fn one_recoloring() {
        let mut actions = ActionMatrix::<T, C, N, E>::from(ColoredGraph::<T, C, N>::red());
        actions.recolor((1, 0), 0);
        for (i, (u,v)) in (0..N).tuple_combinations().enumerate() {
            let slope_0 = actions.slope((0, i));
            let slope_1 = actions.slope((1, i));
            
            match (u, v) {
                (0, 1) => assert_eq!(slope_1, None),
                (0, _) | (1, _) => assert_eq!(slope_1, Some(&choose(N-3, S[0]-2))),
                (_, _) => assert_eq!(slope_1, Some(&(
                    choose(N-2, S[0]-2) - 
                        if N >= 4 && S[0] >= 4 { choose(N-4, S[0].checked_sub(4).unwrap()) }
                        else { 0 }
                ))),
            }

            match (u, v) {
                (0, 1) => assert_eq!(slope_0, Some(&-choose(N-2, S[0]-2))),
                _ => assert_eq!(slope_0, None)
            }
        }
    }
}

impl<const N: usize> From<&Recoloring<N>> for Action {
    fn from(recoloring: &Recoloring<N>) -> Self {
        let pos = edge_to_pos::<N>(recoloring.edge);
        (recoloring.new_color, pos)
    }
}

impl<T: Neighborhood, const C: usize, const N: usize, const E: usize>
ActionMatrix<T, C, N, E> {
    pub fn score(&self) -> Iyy {
        let mut score: Iyy = 0;
        for (color, &s) in S.iter().enumerate() {
            let mut color_score: Iyy = 0;
            for (pos, (u,v)) in (0..N).tuple_combinations().enumerate() {
                let colored_edge = ColoredEdge { color, edge: (u, v) };
                if self.graph.has_edge(colored_edge) {
                    color_score += self.counts[color][pos]
                }
            }
            score += color_score / choose(s, 2)
        };
        score
    }

    pub fn total(&self) -> Iyy {
        self.totals.iter().sum()
    }

    pub fn act(&mut self, (new_color, pos): Action) {
        let edge = pos_to_edge::<N>(pos);
        let old_color = self.graph.color(edge)
            .unwrap();
        self.recolor((new_color, pos), old_color);
        let recoloring = Recoloring { old_color, new_color, edge };
        self.graph.recolor(recoloring);
    }

    pub fn randomly_act(&mut self, rng: &mut ThreadRng) {
        let recoloring = self.graph.random_recoloring(rng);
        let action = Action::from(&recoloring);
        self.recolor(action, recoloring.old_color);
        self.graph.recolor(recoloring)
    }
}

#[cfg(test)]
mod test_random_recoloring {
    use super::*;

    const C: usize = 2;
    const N: usize = 8;
    const E: usize = choose_two(N);

    type T = UxxN<N>;

    #[test]
    fn consistent_counts() {
        let mut actions = ActionMatrix::<T, C, N, E>::from(ColoredGraph::<T, C, N>::red());
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            for c in 0..C {
                let graph_count = actions.graph.count_cliques(c, None, None);
                let matrix_count: Iyy = (0..N)
                    .tuple_combinations()
                    .enumerate()
                    .filter(
                        |(_, (u, v))|
                        {
                            let colored_edge = ColoredEdge { color: c, edge: (*u,*v) };
                            actions.graph.has_edge(colored_edge)
                        })
                    .map(|(pos, _)| {
                        actions.counts[c][pos]
                    })
                    .sum();
                assert_eq!(graph_count * choose(S[c], 2), matrix_count);
            }
            actions.randomly_act(&mut rng);
        }
    }

    impl<T: Neighborhood> ActionMatrix<T, C, N, E> {
        fn calculate_slope(&self, (new_color, pos): Action) -> Option<Iyy> {
            let edge = pos_to_edge::<N>(pos);
            let old_color = self.graph.color(edge)
                .unwrap();
        
            if old_color == new_color { None } 
            else { Some(
                self.counts[old_color][pos] -
                self.counts[new_color][pos]
            )}
        }
    }

    #[test]
    fn verify_all_slopes() {
        let mut actions: ActionMatrix<T, C, N, E> = ActionMatrix::from(ColoredGraph::red());
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            for c in 0..C {
                for pos in 0..E {
                    let slope = actions.slope((c, pos))
                        .map(|&x| x);
                    let calculated_slope = actions.calculate_slope((c, pos));
                    assert_eq!(slope, calculated_slope)
                }
            }
            actions.randomly_act(&mut rng)
        }
    }

    #[test]
    fn consistent_scores() {
        let mut actions = ActionMatrix::<T, C, N, E>::from(ColoredGraph::<T, C, N>::red());
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            assert_eq!(actions.score(), actions.graph.score());
            assert_eq!(actions.score(), actions.total());
            actions.randomly_act(&mut rng)
        }
    }
}

impl<T: Neighborhood, const C: usize, const N: usize, const E: usize>
PartialEq for ActionMatrix<T, C, N, E> {
    fn eq(&self, other: &Self) -> bool {
        self.graph == other.graph
    }
}