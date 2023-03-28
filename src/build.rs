/* see https://stackoverflow.com/a/37528134 */

use std::{env, fs::File, io::Write, path::Path};
fn main() {
    let out_dir = env::var("OUT_DIR").expect("No out dir");
    let dest_path = Path::new(&out_dir).join("constants.rs");
    let mut f = File::create(&dest_path).expect("Could not create file");
    
    let n = option_env!("N")
        .map_or(Ok(5), str::parse)
        .expect("Could not parse N");

    write!(&mut f, "pub const N: usize = {n};\n")
        .expect("Could not write file");
    println!("cargo:rerun-if-env-changed=N");

    let uxx = match n {
        n if n <= 8 => "u8",
        n if n <= 16 => "u16",
        n if n <= 32 => "u32",
        n if n <= 64 => "u64",
        n if n <= 128 => "u128",
        _ => panic!("N = {n} is too large")
    };

    let iyy = match n {
        n if n <= 16 => "i16",
        n if n <= 32 => "i32",
        _ => "i64"
    };

    let uzz = match n {
        n if n <= 48 => "u32",
        _ => "u64"
    };

    write!(&mut f, "pub type Uxx = {uxx};\npub type Iyy = {iyy};\npub type Uzz = {uzz};\n")
        .expect("Could not write file");
    println!("cargo:rerun-if-env-changed=N");

    let epochs = option_env!("EPOCHS")
        .map_or(Ok(50), str::parse)
        .expect("Could not parse EPOCHS");

    write!(&mut f, "pub const EPOCHS: usize = {epochs};\n")
        .expect("Could not write file");
    println!("cargo:rerun-if-env-changed=EPOCHS");

    let episodes = option_env!("EPISODES")
        .map_or(Ok(10_000), str::parse)
        .expect("Could not parse EPISODES");

    write!(&mut f, "pub const EPISODES: Uzz = {episodes};\n")
        .expect("Could not write file");
    println!("cargo:rerun-if-env-changed=EPISODES");

    let explore = option_env!("EXPLORE")
        .map_or(Ok(0.2), str::parse)
        .expect("Could not parse EXPLORE");

    write!(&mut f, "pub const EXPLORE: f64 = {explore};\n")
        .expect("Could not write file");
    println!("cargo:rerun-if-env-changed=EXPLORE");

    let s: Vec<usize> = option_env!("S")
        .unwrap_or("[3, 3]")
        .split(|c: char| !c.is_numeric())
        .filter(|s| s.len() > 0)
        .map(|s| s.parse().unwrap())
        .collect();
    
    write!(&mut f, "pub const S: [usize; {}] = {s:?};\n", s.len())
        .expect("Could not write file");
    println!("cargo:rerun-if-env-changed=S");
}