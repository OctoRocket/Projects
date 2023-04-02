mod args;
mod functions;

use args::Args;
use clap::Parser;
use functions::{
    ConvertToSets,
    Sets,
};
use rand::random;

fn main() {
    let args = Args::parse();
    let sets = args.chars.convert_to_sets();
    let mut password = String::new();
    for _ in 0..args.length {
        let set = &sets[random::<usize>() % sets.len()];
        password.push(match set {
            Sets::Lowercase => (random::<u8>() % 26 + b'a') as char,
            Sets::Uppercase => (random::<u8>() % 26 + b'A') as char,
            Sets::Digits => (random::<u8>() % 10 + b'0') as char,
            Sets::Symbols => {
                let symbols = b"!@#$%^&*()_+-=[]{};':\",./<>?`~";
                symbols[random::<usize>() % symbols.len()] as char
            }
        });
    }
    println!("{}", password);
}
