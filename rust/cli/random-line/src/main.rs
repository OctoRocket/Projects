use std::{
    fs::read_to_string,
    env::args,
};
use rand::{
    seq::IteratorRandom,
    thread_rng,
};

fn main() {
    println!("{}", match match read_to_string(match args().nth(1) {
        Some(s) => s,
        _ => {
            println!("Invalid file path.");
            return;
        }
    }) {
        Ok(f) => f,
        _ => {
            println!("Invalid file path.");
            return;
        }
    }.split("\n").choose(&mut thread_rng()) {
        Some(s) => s,
        _ => {
            println!("File empty.");
            return;
        }
    });
}
