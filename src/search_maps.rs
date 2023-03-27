use std::collections::HashMap;
use rand::{rngs::ThreadRng, seq::SliceRandom};

use crate::{
    colored_graph::*,
    action_matrix::*
};

const C_NOISE: f64 = 0.07;   // todo("what to do with this value?")

pub struct ScoreKeeper<const N: usize> {
    roots: Vec<ActionMatrix<N>>,
    best_count: Iyy
}

impl<const N: usize> From<ActionMatrix<N>> for ScoreKeeper<N> {
    fn from(actions: ActionMatrix<N>) -> Self {
        let count = actions.total();
        ScoreKeeper { roots: vec![actions], best_count: count }
    }
}

impl<const N: usize> ScoreKeeper<N> {
    pub fn random_root(&self, rng: &mut ThreadRng) -> &ActionMatrix<N> { 
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

impl<const N: usize> ScoreKeeper<N> {
    #[must_use]
    pub fn update(&mut self, actions: &ActionMatrix<N>) -> ScoreUpdate {
        let count = actions.total();
        match self.best_count.cmp(&count) {
            std::cmp::Ordering::Less => ScoreUpdate::Worse,
            std::cmp::Ordering::Equal => {
                if !self.roots.contains(actions) {
                    const MAX_N_ROOTS: usize = 250;
                    match self.roots.len().cmp(&MAX_N_ROOTS) {
                        std::cmp::Ordering::Less => {
                            self.roots.push(actions.clone());
                            print!("\r{} minima... ", self.roots.len())
                        }
                        std::cmp::Ordering::Equal => {
                            self.roots.push(actions.clone());
                            println!("\r{MAX_N_ROOTS}+ minima... ")
                        }
                        std::cmp::Ordering::Greater => {}
                    }
                    ScoreUpdate::Tie
                }
                else {
                    ScoreUpdate::Known
                }
            },
            std::cmp::Ordering::Greater => {
                self.roots = vec![actions.clone()];
                self.best_count = count;
                println!("score improved to {count} by");
                //self.root.show_neighborhoods();
                self.roots[0].graph().show_matrix();
                println!();
                print!("\r{} minimum... ", self.roots.len());
                if count == 0 {
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

pub struct GraphData {
    n_visits: Uzz,
    action_map: ActionMap
}

impl Default for GraphData {
    fn default() -> Self {
        Self { n_visits: 0, action_map: Default::default() }
    }
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
        C_NOISE * (self.n_visits as f64).sqrt()
    }
    
    fn mu(&self, action: &Action) -> Option<f64> {
        let value = self.action_map.actions.get(action);
        let (q_ga, n_ga) = match value {
            Some(&value) => value,
            None => return None
        };
        
        Some(
            q_ga as f64 + 
            C_NOISE * (self.n_visits as f64).sqrt()
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
pub struct GraphMap<const N: usize> {
    graphs: HashMap<ColoredGraph<N>, GraphData>
}

impl<const N: usize> GraphMap<N> {
    pub fn next_action(
        &mut self,
        actions: &mut ActionMatrix<N>,
        score_keeper: &mut ScoreKeeper<N>,
        // seen_edges: &mut [bool; E]
    ) -> Option<ScoreUpdate> {
        let graph_data = self.graphs.entry(actions.graph().clone())
            .or_insert(GraphData::default());
        let best_visited = graph_data.visited_argmax(); //seen_edges);
        let default_nu = graph_data.default_nu();

        // todo!("would be nice to implement this with a general predicate in the priority_queue crate")
        let action_queue = actions.actions_mut();
        let mut popped_actions = vec![];

        let (best_action, q_ga) = loop {
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
        graph_data.record(best_action, q_ga);
        Some(score_keeper.update(actions))

    }
}