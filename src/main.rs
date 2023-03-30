use std::io;
use mcts_ramsey::{
    learning_loop::search,
    colored_graph::{
        N, S, choose_two,
        EPOCHS, EPISODES, ROOTS, EXPLORE, GUESS_P}};

const C: usize = S.len();
const E: usize = choose_two(N);

fn main() {
    println!("Goal: prove R{S:?} > {N}.");
    println!("EPOCHS   = {EPOCHS}");
    println!("EPISODES = {EPISODES}");
    println!("ROOTS    = {ROOTS}");
    println!("EXPLORE  = {EXPLORE}");
    println!("GUESS_P  = {GUESS_P:?}");
    println!("Enter 'q' now to quit.");

    use std::time::Instant;
    let now = Instant::now();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    if input.contains('q') { return }

    search::<C, N, E>();

    let elapsed = now.elapsed();
    println!("Elapsed: {:.3?}", elapsed);
}
