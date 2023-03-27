use mcts_ramsey::{learning_loop::search, colored_graph::Uzz, colored_graph::{S, choose_two}};

const C: usize = S.len();
const N: usize = 8;
const E: usize = choose_two(N);
const N_EPOCHS: usize = 50;
const N_EPISODES: Uzz = 10_000;

fn main() {
    search::<C, N, E, N_EPOCHS, N_EPISODES>();
}
