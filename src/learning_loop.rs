use crate::{prelude::*, neighborhood::Neighborhood};

use rand::distributions::WeightedIndex;

use crate::{search_map::*, action_matrix::*, colored_graph::*};

pub fn play_episode<T: Neighborhood, const C: usize, const N: usize, const E: usize>
(g_map: &mut GraphMap<T, C, N, E>, score_keeper: &mut ScoreKeeper<T, C, N, E>, n_moves: usize) -> Result<(), ScoreUpdate>
{
    let mut rng = rand::thread_rng();
    let mut chosen_root = score_keeper.random_root(&mut rng).clone();
    let mut action_matrix = chosen_root.clone();
    let mut actions_taken = vec!();
    for _ in 0..n_moves {
        match g_map.next_action(&mut action_matrix) {
            Some(action) => actions_taken.push(action),
            None => todo!("refactor the return type")
        }
    }

    g_map.update_counts(score_keeper,&mut chosen_root, actions_taken)
}

pub fn play_epoch<T: Neighborhood, const C: usize, const N: usize, const E: usize>(
    g_map: &mut GraphMap<T, C, N, E>,
    score_keeper: &mut ScoreKeeper<T, C, N, E>,
    n_moves: usize,
    n_episodes: Uzz
) -> Option<ScoreUpdate>
{
    for i in 1..(n_episodes+1) {
        if i % (10 * EPISODES) == 0 { println!("== EPISODE == {i}") }
        if let Err(ScoreUpdate::Done) = play_episode(g_map, score_keeper, n_moves) {
            return Some(ScoreUpdate::Done)
        }
    }
    None
}

pub fn play_epochs<T: Neighborhood, const C: usize, const N: usize, const E: usize>
(g_map: &mut GraphMap<T, C, N, E>, score_keeper: &mut ScoreKeeper<T, C, N, E>)
{
    for epoch in 1..(EPOCHS+1) {
        println!("==== EPOCH ==== {epoch}");
        if let Some(ScoreUpdate::Done) = play_epoch::<T, C, N, E>(g_map, score_keeper, E/4 + epoch, EPISODES) {
            println!("R{S:?} > {N}");
            return
        }
    }
}

pub fn search<T: Neighborhood, const C: usize, const N: usize, const E: usize>()
{
    let mut rng = rand::thread_rng();
    let dist = WeightedIndex::new(&GUESS_P)
        .unwrap();
    let graph = ColoredGraph::<T, C, N>::random(&mut rng, &dist);
    let actions = ActionMatrix::from(graph);
    
    let mut score_keeper = ScoreKeeper::from(actions);
    let mut g_map = GraphMap::default();
    play_epochs::<T, C, N, E>(&mut g_map, &mut score_keeper);
}
