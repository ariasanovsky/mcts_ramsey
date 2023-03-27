use mcts_ramsey::{learning_loop::search, colored_graph::Uzz};
fn main() {
    const N: usize = 8;
    const N_EPOCHS: usize = 50;
    const N_EPISODES: Uzz = 10_000;
    search::<N, N_EPOCHS, N_EPISODES>();
}
