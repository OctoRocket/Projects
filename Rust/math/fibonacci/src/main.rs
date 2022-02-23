use num::BigUint;
use num::{Zero, One};
use std::io;

fn fib(len: u32) -> Vec<BigUint> {
    println!();
    let mut a: BigUint = Zero::zero();
    let mut b: BigUint = One::one();
    let mut c: BigUint;
    let mut vec: Vec<BigUint> = Vec::new();
    for _n in 1..=len {
        vec.push(b.clone());
        c = a + b.clone();
        a = b;
        b = c;
    }
    return vec;
}

fn main() {
    println!("Enter the number of numbers of the fib seq that you want.");
    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read line.");
    let inp = inp.trim().parse::<u32>().expect("Failed to parse");
    let vec = fib(inp);
    for n in 0..vec.len() {
        if n < vec.len() - 1 {
            print!("{}, ", vec[n])
        } else {
            println!("{}", vec[n])
        }
    }
}
