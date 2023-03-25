use std::collections::HashMap;
use crate::{
    colored_graph::*,
    action_matrix::*
};

pub struct ScoreKeeper {
    root: ColoredGraph,
    best_count: Iyy
}

impl From<ColoredGraph> for ScoreKeeper {
    fn from(graph: ColoredGraph) -> Self {
        let count = (0..C)
            .map(|c| graph.count_cliques(c, None, None))
            .sum();
        ScoreKeeper { root: graph, best_count: count }
    }
}

impl ScoreKeeper {
    pub fn root(&self) -> &ColoredGraph { &self.root }
}

pub enum ScoreUpdate {
    Better,
    Worse,
    Tie
}

impl ScoreKeeper {
    pub fn update(&mut self, graph: &ColoredGraph) -> ScoreUpdate {
        let count = (0..C).map(|c| 
            graph.count_cliques(c, None, None)
        ).sum(); // todo("make function && dynamically track score...")
        match self.best_count.cmp(&count) {
            std::cmp::Ordering::Less => ScoreUpdate::Worse,
            std::cmp::Ordering::Equal => ScoreUpdate::Tie,
            std::cmp::Ordering::Greater => {
                self.root = graph.clone();
                self.best_count = count;
                println!("score improved to {count} by");
                self.root.show_neighborhoods();
                ScoreUpdate::Better
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

const C_NOISE: f64 = 0.1;

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

    pub fn visited_argmax(&self) -> Option<(Action, f64)> {
        let mut argmax = None;
        for action in self.action_map.actions.keys() {
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

pub struct GraphMap {
    graphs: HashMap<ColoredGraph, GraphData>
}

impl GraphMap {
    pub fn next_action(
        &mut self,
        actions: &mut ActionMatrix,
        score_keeper: &mut ScoreKeeper
    ) -> Action {
        let graph_data = self.graphs.entry(actions.graph().clone())
            .or_insert(GraphData::default());
        let best_visited = graph_data.visited_argmax();
        let default_nu = graph_data.default_nu();

        // todo!() would be nice to implement this with a general predicate in the priority_queue crate
        let mut best_unvisited: Option<(Action, Iyy)> = None;
        let action_queue = actions.actions_mut();
        let mut popped_actions = vec![];
        loop {
            let Some((action, q_ga)) = action_queue.peek()
                else { break };
            if graph_data.action_map.actions.contains_key(action) { // todo!("make a seen function")
                popped_actions.push(action_queue.pop().unwrap());
            }
            else {
                best_unvisited = Some((*action, *q_ga))
            }
        }

        while let Some((action, q_ga)) = popped_actions.pop() {
            action_queue.push(action, q_ga);
        }

        let (best_action, q_ga) = match (best_visited, best_unvisited) {
            (None, None) => panic!("Couldn't find a best visited or unvisited action!"),
            (None, Some((action, q_ga))) => (action, Some(q_ga)),
            (Some((action, _)), None) => (action, None),
            (Some((v_action, mu_ga)), 
            Some((u_action, q_ga))) => {
                if mu_ga >= q_ga as f64 + default_nu {
                    (v_action, None)
                }
                else {
                    (u_action, Some(q_ga))
                }
            }
        };
        // println!("we removed {} items from the action queue... is this healthy?", popped_actions.len());

        actions.act(best_action);
        score_keeper.update(actions.graph());

        graph_data.record(best_action, q_ga);
        best_action
    }
}