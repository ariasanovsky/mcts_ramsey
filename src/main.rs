use std::io;
use mcts_ramsey::{learning_loop::search, colored_graph::Uzz, colored_graph::{S, choose_two, N}};

const C: usize = S.len();
const E: usize = choose_two(N);
const N_EPOCHS: usize = 50;
const N_EPISODES: Uzz = 10_000;

fn main() {
    println!("Goal: prove R{S:?} > {N}.\nEnter 'q' now to quit.");
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    if input.contains('q') { return }

    search::<C, N, E, N_EPOCHS, N_EPISODES>();
}
