use std::io;
use mcts_ramsey::{
    learning_loop::search,
    colored_graph::{
        N, S, choose_two,
        EPOCHS, EPISODES, ROOTS, EXPLORE, GUESS_P}};

const C: usize = S.len();
const E: usize = choose_two(N);

fn main() {

    let args: Vec<String> = std::env::args().collect();
    if args.contains(&String::from("clean")) {
        /* https://stackoverflow.com/a/69987121 */
        for path in std::fs::read_dir("./plots/").unwrap() {
            let path = path.unwrap().path();
            let extension = path.extension().unwrap();
            use std::ffi::OsStr;
            if extension == OsStr::new("svg") {
                std::fs::remove_file(path).unwrap();
            }
        }
        println!("plots/*.svg cleared 😊")
    }
    else {
        println!("To clear plots/*.svg, pass 'clean' as an argument 😊")
    }

    println!("Goal: prove R{S:?} > {N}.");
    println!("EPOCHS   = {EPOCHS}");
    println!("EPISODES = {EPISODES}");
    println!("ROOTS    = {ROOTS}");
    println!("EXPLORE  = {EXPLORE}");
    println!("GUESS_P  = {GUESS_P:?}");
    println!("Enter 'q' now to quit.");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    if input.contains('q') { return }

    use std::time::Instant;
    let now = Instant::now();

    search::<C, N, E>();

    let elapsed = now.elapsed();
    println!("Elapsed: {elapsed:.3?}");
}
