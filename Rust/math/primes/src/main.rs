use std::io;

fn input() -> u64{
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).expect("Failed to read line");
    let inp = inp.trim().parse::<u64>().expect("Failed to parse"); 
    inp
}

fn primes(inp: u64) -> Vec<u64> {
    let mut prime_list = vec![2u64];
    let mut failed = false;
    for i in 2..inp+1 {
        for q in &prime_list {
            if i == 1 {
                break;
            } else if i % q == 0 {
                failed = true;
                break;
            }
        }
        if failed == false {
            prime_list.push(i);
        }
        failed = false;
    }
    prime_list
}

fn main() {
    let inp = input();
    for i in primes(inp) {
        print!("{} ", i);
    }
    println!();
}
