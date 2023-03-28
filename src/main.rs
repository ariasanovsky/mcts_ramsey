use std::io;
use mcts_ramsey::{
    learning_loop::search,
    colored_graph::{S, choose_two, N, EPOCHS, EPISODES, EXPLORE}};

const C: usize = S.len();
const E: usize = choose_two(N);

fn main() {
    println!("Goal: prove R{S:?} > {N}.");
    println!("EPOCHS = {EPOCHS}, EPISODES = {EPISODES}");
    println!("EXPLORE = {EXPLORE}");
    println!("Enter 'q' now to quit.");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    if input.contains('q') { return }

    search::<C, N, E, EPOCHS, EPISODES>();
}
