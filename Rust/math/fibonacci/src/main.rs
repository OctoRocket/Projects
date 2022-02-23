use std::io;

fn fib(len: u32) -> Vec<u128> {
    println!();
    let mut a: u128 = 0;
    let mut b: u128 = 1;
    let mut c: u128;
    let mut vec: Vec<u128> = Vec::new();
    for _n in 1..=len {
        vec.push(b);
        c = a + b;
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
