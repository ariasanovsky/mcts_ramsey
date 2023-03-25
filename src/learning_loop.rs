use crate::{search_maps::*, action_matrix::*};

pub fn play_episode(g_map: &mut GraphMap, score_keeper: &mut ScoreKeeper, budget: i32) -> Vec<Action> {
    let mut actions: Vec<Action> = vec!();
    let mut action_matrix = ActionMatrix::from(score_keeper.root().clone());
    
    for _ in 0..budget {
        let recoloring = g_map.next_action(&mut action_matrix, score_keeper);
        actions.push(recoloring);
    }
    return actions
}

pub fn play_epoch(g_map: &mut GraphMap, score_keeper: &mut ScoreKeeper, budget: i32, n_episodes: i32) {
    for i in 1..(n_episodes+1) {
        if i % 1_000 == 0 { println!("== EPISODE == {i}") }
        play_episode(g_map, score_keeper, budget);
    }
}

pub fn play_epochs(g_map: &mut GraphMap, score_keeper: &mut ScoreKeeper, max_budget: i32, n_episodes: i32) {
    for budget in 1..max_budget {
        println!("==== EPOCH ==== {budget}");
        play_epoch(g_map, score_keeper, budget, n_episodes);
    }
}