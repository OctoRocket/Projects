use std::env::args;

fn main() {
    println!("{}", args().collect::<Vec<String>>()[1].len());
}
