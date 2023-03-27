use mcts_ramsey::{learning_loop::search, colored_graph::Uzz, colored_graph::S};

const C: usize = S.len();
const N: usize = 8;
const N_EPOCHS: usize = 50;
const N_EPISODES: Uzz = 10_000;

fn main() {
    search::<C, N, N_EPOCHS, N_EPISODES>();
}
