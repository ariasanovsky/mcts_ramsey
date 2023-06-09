mod tests;

use std::marker::PhantomData;
use priority_queue::PriorityQueue;

use crate::{prelude::*, colored_graph::neighborhood::*};
use crate::colored_graph::*;

use itertools::Itertools;
use rand::rngs::ThreadRng;


pub type EdgePos = usize;
pub type Action = (Color, EdgePos);

#[derive(Clone)]
pub struct ActionMatrix<T: Neighborhood, const C: usize, const N: usize, const E: usize> {
    pub(crate) counts: [[Iyy; E]; C],
    pub(crate) graph: ColoredGraph<T, C, N>,
    pub(crate) actions: PriorityQueue<Action, Iyy>,
    pub(crate) totals: [Iyy; C],
    pub(crate) phantom: PhantomData<T>
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

impl<T: Neighborhood, const C: usize, const N: usize, const E: usize>
ActionMatrix<T, C, N, E> {
    pub fn graph(&self) -> &ColoredGraph<T, C, N> { &self.graph }
    pub fn actions_mut(&mut self) -> &mut PriorityQueue<Action, Iyy> { &mut self.actions }
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

impl<T: Neighborhood, const C: usize, const N: usize, const E: usize>
PartialEq for ActionMatrix<T, C, N, E> {
    fn eq(&self, other: &Self) -> bool {
        self.graph == other.graph
    }
}