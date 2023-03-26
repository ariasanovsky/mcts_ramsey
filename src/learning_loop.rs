use crate::{search_maps::*, action_matrix::*, colored_graph::*};

pub fn play_episode(g_map: &mut GraphMap, score_keeper: &mut ScoreKeeper, budget: Uzz) -> Option<ScoreUpdate> {
    let mut action_matrix = score_keeper.root().clone();
    /* let mut seen_edges = [false; E]; */
    // todo("clone the whole ActionMatrix instead?")
    for _ in 0..E {
        match g_map.next_action(&mut action_matrix, score_keeper, /* &mut seen_edges */) {
            Some(ScoreUpdate::Done) => return Some(ScoreUpdate::Done),
            _ => {}
        }
    }
    return None
}

pub fn play_epoch(g_map: &mut GraphMap, score_keeper: &mut ScoreKeeper, budget: Uzz, n_episodes: Uzz) -> Option<ScoreUpdate> {
    for i in 1..(n_episodes+1) {
        if i % 100_000 == 0 { println!("== EPISODE == {i}") }
        match play_episode(g_map, score_keeper, budget) {
            Some(ScoreUpdate::Done) => return Some(ScoreUpdate::Done),
            _ => {}
        }
    }
    None
}

pub fn play_epochs(g_map: &mut GraphMap, score_keeper: &mut ScoreKeeper, max_budget: Uzz, n_episodes: Uzz) {
    for budget in 1..max_budget {
        println!("==== EPOCH ==== {budget}");
        match play_epoch(g_map, score_keeper, budget, n_episodes) {
            Some(ScoreUpdate::Done) => return,
            _ => {}
        }
    }
}

pub fn search(max_budget: Uzz, n_episodes: Uzz) {
    let mut g_map = GraphMap::default();
    let mut rng = rand::thread_rng();
    let graph = ColoredGraph::uniformly_random(&mut rng);
    let actions = ActionMatrix::from(graph);
    let mut score_keeper = ScoreKeeper::from(actions);
    play_epochs(&mut g_map, &mut score_keeper, max_budget, n_episodes);
}
