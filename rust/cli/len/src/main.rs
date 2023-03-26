use std::env::args;

fn main() {
    let arg = &args().collect::<Vec<String>>()[1];
    println!("{}", arg.len());
}
