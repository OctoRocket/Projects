use std::io;

fn read_line() -> f64 {
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Failed to read line");
    let x = x.trim().parse::<f64>().expect("Failed to parse");
    x
}

fn main() {
    let a = read_line();
    let b = read_line();
    let c = read_line();
    let x = (-b + ((b * b) - 4.0 * a * c).sqrt()) / 2.0 * a;
    let y = (-b - ((b * b) - 4.0 * a * c).sqrt()) / 2.0 * a;
    println!("{}, {}", x, y);
}
