use std::env::args;

fn primes_up_to(limit: u64) -> Vec<u64> {
    let mut primes = vec![];
    let mut potential_primes = (2..=limit).rev().collect::<Vec<_>>();

    while let Some(number) = potential_primes.pop() {
        primes.push(number);
        potential_primes.retain(|n| n % number != 0);
    }

    primes
}

fn main() {
    for i in primes_up_to(
        match args().collect::<Vec<_>>().get(1) {
            Some(v) => match v.parse::<u64>() {
                Ok(v) => v,
                Err(_) => {
                    println!("Couldn't parse input (did you provide a number?)");
                    return;
                }
            }
            None => {
                println!("Please provide a number.");
                return;
            }
        }
    ) {
        println!("{}", i);
    }
}
