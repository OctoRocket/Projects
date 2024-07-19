use std::io::{self, Write};

fn read_line() -> f64 {
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Failed to read line");
    let x = x.trim().parse::<f64>().expect("Failed to parse");
    x
}

fn main() {
    print!("A: ");
    io::stdout().flush().unwrap();
    let a = read_line();
    print!("B: ");
    io::stdout().flush().unwrap();
    let b = read_line();
    print!("C: ");
    io::stdout().flush().unwrap();
    let c = read_line();
    // quadratic formula
    let x = (-b + ((b * b) - 4.0 * a * c).sqrt()) / 2.0 * a;
    let y = (-b - ((b * b) - 4.0 * a * c).sqrt()) / 2.0 * a;
    println!("{}, {}", x, y);
}

