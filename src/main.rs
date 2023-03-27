use mcts_ramsey::{learning_loop::search};

fn main() {
    search::<8>(50, 10_000);
}
