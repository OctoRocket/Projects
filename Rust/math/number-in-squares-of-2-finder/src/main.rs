use std::{io, u128};


fn input() -> String {
    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read line");
    inp
}

fn main() {
    println!("Enter number to find:");
    let num = input().trim().parse::<u128>().expect("Failed to read line");
    let mut c = 1;
    loop {
        ...
    }
}
