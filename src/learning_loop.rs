use crate::{search_maps::*, action_matrix::*, colored_graph::*};

pub fn play_episode<const N: usize>
(g_map: &mut GraphMap<N>, score_keeper: &mut ScoreKeeper<N>, n_moves: usize) -> Option<ScoreUpdate>
{
    let mut rng = rand::thread_rng();
    let mut action_matrix = score_keeper.random_root(&mut rng).clone();
    /* let mut seen_edges = [false; E]; */
    // todo("clone the whole ActionMatrix instead?")
    for _ in 0..n_moves {
        match g_map.next_action(&mut action_matrix, score_keeper, /* &mut seen_edges */) {
            Some(ScoreUpdate::Done) => return Some(ScoreUpdate::Done),
            _ => {}
        }
    }
    return None
}

pub fn play_epoch<const N: usize, const N_EPISODES: Uzz>
(g_map: &mut GraphMap<N>, score_keeper: &mut ScoreKeeper<N>, n_moves: usize, n_episodes: Uzz) -> Option<ScoreUpdate>
{
    for i in 1..(n_episodes+1) {
        if i % (10 * N_EPISODES) == 0 { println!("== EPISODE == {i}") }
        match play_episode(g_map, score_keeper, n_moves) {
            Some(ScoreUpdate::Done) => return Some(ScoreUpdate::Done),
            _ => {}
        }
    }
    None
}

pub fn play_epochs<const N: usize, const N_EPOCHS: usize, const N_EPISODES: Uzz>
(g_map: &mut GraphMap<N>, score_keeper: &mut ScoreKeeper<N>)
{
    for epoch in 0..N_EPOCHS {
        println!("==== EPOCH ==== {epoch}");
        match play_epoch::<N, N_EPISODES>(g_map, score_keeper, epoch + N, N_EPISODES) {
            Some(ScoreUpdate::Done) => {
                println!("R{S:?} > {N}");
                return
            },
            _ => {}
        }
    }
}

use rand::distributions::WeightedIndex;

pub fn search<const N: usize, const N_EPOCHS: usize, const N_EPISODES: Uzz>()
{
    let mut rng = rand::thread_rng();
    let dist = WeightedIndex::new(&P).unwrap();
    let graph = ColoredGraph::<N>::random(&mut rng, &dist);
    let actions = ActionMatrix::from(graph);
    
    let mut score_keeper = ScoreKeeper::from(actions);
    let mut g_map = GraphMap::default();
    play_epochs::<N, N_EPOCHS, N_EPISODES>(&mut g_map, &mut score_keeper);
}
