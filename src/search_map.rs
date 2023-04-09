use crate::neighborhood::Neighborhood;
use crate::prelude::*;
use crate::{colored_graph::*, action_matrix::*};

use std::collections::HashMap;
use rand::{rngs::ThreadRng, seq::SliceRandom};



pub struct ScoreKeeper<T: Neighborhood, const C: usize, const N: usize, const E: usize> {
    roots: Vec<ActionMatrix<T, C, N, E>>,
    best_count: Iyy,
    name: String
}

impl<T: Neighborhood, const C: usize, const N: usize, const E: usize>
From<ActionMatrix<T, C, N, E>> for ScoreKeeper<T, C, N, E> {
    fn from(actions: ActionMatrix<T, C, N, E>) -> Self {
        let count = actions.total();
        let name = format!("r{S:?}_{N}");
        ScoreKeeper { roots: vec![actions], best_count: count, name }
    }
}

impl<T: Neighborhood, const C: usize, const N: usize, const E: usize>
ScoreKeeper<T, C, N, E> {
    pub fn random_root(&self, rng: &mut ThreadRng) -> &ActionMatrix<T, C, N, E> { 
        self.roots.choose(rng).unwrap()
    }
}

pub enum ScoreUpdate {
    Done,
    Better,
    Tie,
    Known,
    Worse
}

impl<T: Neighborhood, const C: usize, const N: usize, const E: usize>
ScoreKeeper<T, C, N, E> {
    #[must_use]
    pub fn update(&mut self, actions: &ActionMatrix<T, C, N, E>) -> ScoreUpdate {
        let count = actions.total();
        match self.best_count.cmp(&count) {
            std::cmp::Ordering::Less => ScoreUpdate::Worse,
            std::cmp::Ordering::Equal => {
                if !self.roots.contains(actions) {
                    match self.roots.len().cmp(&ROOTS) {
                        std::cmp::Ordering::Less => {
                            self.roots.push(actions.clone());
                            print!("\r{} minima... ", self.roots.len())
                        }
                        std::cmp::Ordering::Equal => {
                            self.roots.push(actions.clone());
                            println!("\r{ROOTS}+ minima... ")
                        }
                        std::cmp::Ordering::Greater => {}
                    }
                    if self.best_count == 0 {
                        ScoreUpdate::Done
                    }
                    else {
                        ScoreUpdate::Tie
                    }
                }
                else {
                    ScoreUpdate::Known
                }
            },
            std::cmp::Ordering::Greater => {
                self.roots = vec![actions.clone()];
                self.best_count = count;
                println!("score improved to {count} by");
                if N <= 10 {
                    self.roots[0].graph().show_neighborhoods();
                }
                if N <= 25 {
                    self.roots[0].graph().show_matrix();
                    println!();
                }
                println!("{:?}", self.roots[0].graph().graph6s());
                let docs = actions
                    .graph()
                    .svg(self.name.clone());
                docs.render();
                
                print!("\r{} minimum... ", self.roots.len());
                if count == 0 {
                    println!("==== DONE ====\nCheck out plots/{}*.svg 😊", self.name);
                    ScoreUpdate::Done
                }
                else {
                    ScoreUpdate::Better
                }
            }
        }
    }
}

#[derive(Default)]
pub struct ActionMap {
    actions: HashMap<Action, (Iyy, Uzz)>
}

#[derive(Default)]
pub struct GraphData {
    n_visits: Uzz,
    action_map: ActionMap
}

impl GraphData {
    pub fn record(&mut self, action: Action, q_ga: Option<Iyy>) {
        self.n_visits += 1;
        let res = self.action_map.actions.get_mut(&action);
        match res {
            Some((_, n_ga)) => *n_ga += 1,
            None => {self.action_map.actions.insert(action, (q_ga.unwrap(), 1));}
        }
    }
    
    pub fn default_nu(&self) -> f64 {
        EXPLORE * (self.n_visits as f64).sqrt()
    }
    
    fn mu(&self, action: &Action) -> Option<f64> {
        let value = self.action_map.actions.get(action);
        let (q_ga, n_ga) = match value {
            Some(&value) => value,
            None => return None
        };
        
        Some(
            q_ga as f64 + 
            EXPLORE * (self.n_visits as f64).sqrt()
            / ((1 + n_ga) as f64)
        )
    }

    pub fn visited_argmax(&self, /* seen_edges: &[bool; E]*/ ) -> Option<(Action, f64)> {
        let mut argmax = None;
        for action in self.action_map.actions.keys() {
            /* if seen_edges[action.1] { continue } */
            let mu = self.mu(action)
                .unwrap();
            match argmax {
                Some((_, max_mu)) => {
                    if max_mu < mu {
                        argmax = Some((*action, mu))
                    }
                },
                None => argmax = Some((*action, mu))
            }
        }
        argmax
    }
}

#[derive(Default)]
pub struct GraphMap<T: Neighborhood, const C: usize, const N: usize, const E: usize> {
    graphs: HashMap<ColoredGraph<T, C, N>, GraphData>
}

impl<T: Neighborhood, const C: usize, const N: usize, const E: usize>
GraphMap<T, C, N, E>
{
    pub fn next_action(
        &self,
        actions: &mut ActionMatrix<T, C, N, E>,
        score_keeper: &mut ScoreKeeper<T, C, N, E>,
    ) -> Option<(ScoreUpdate, Action)>
    {
        let default_graph_data = GraphData::default();
        let graph_data = self
            .graphs
            .get(&actions.graph);
        let graph_data = if graph_data.is_some() {
            graph_data.clone().unwrap()
        }
        else {
            &default_graph_data
        };
        let graph_data = graph_data;
        let best_visited = graph_data.visited_argmax();
        let default_nu = graph_data.default_nu();

        // todo!("would be nice to implement this with a general predicate in the priority_queue crate")
        let action_queue = actions.actions_mut();
        let mut popped_actions = vec![];

        let (best_action, _) = loop {
            let best_unvisited: Option<(Action, Iyy)> = loop {
                let Some((action, q_ga)) = action_queue.peek()
                    else { break None };
                if graph_data.action_map.actions.contains_key(action) { // todo!("make a seen function")
                    popped_actions.push(action_queue.pop().unwrap());
                }
                else { // if !seen_edges[action.1] {
                    break Some((*action, *q_ga))
                }
            };

            while let Some((action, q_ga)) = popped_actions.pop() {
                action_queue.push(action, q_ga);
            }

            match (best_visited, best_unvisited) {
                (None, None) => {
                    // for pos in 0..E { seen_edges[pos] = false }
                    // continue
                    panic!("Couldn't find an action!")
                }
                (None, Some((action, q_ga))) => break (action, Some(q_ga)),
                (Some((action, _)), None) => break (action, None),
                (Some((v_action, mu_ga)), 
                Some((u_action, q_ga))) => {
                    if mu_ga >= q_ga as f64 + default_nu {
                        break (v_action, None)
                    }
                    else {
                        break (u_action, Some(q_ga))
                    }
                }
            };
        };
        // println!("we removed {} items from the action queue... is this healthy?", popped_actions.len());
        // seen_edges[best_action.1] = true;
        actions.act(best_action);
        Some((score_keeper.update(actions), best_action))

    }

    pub fn update_counts(&mut self, chosen_root: &mut ActionMatrix<T, C, N, E>, actions_taken: Vec<Action>) {
        let graph_data = self.graphs.entry(chosen_root.graph().clone())
            .or_insert(GraphData::default());
        graph_data.n_visits += 1;

        for best_action in actions_taken {
            let graph_data = self.graphs.entry(chosen_root.graph().clone())
                .or_insert(GraphData::default());
            let q_ga = chosen_root.counts[best_action.0][best_action.1];
            graph_data.record(best_action, Some(q_ga));
            chosen_root.act(best_action);
        }
    }
    
}