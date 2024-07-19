use std::{env::args, io::{stdout, Write}, time::Instant};

fn primes_up_to(limit: u64) -> Vec<u64> {
    let mut primes = vec![];
    let mut potential_primes = (2..=limit).rev().collect::<Vec<_>>();

    while let Some(number) = potential_primes.pop() {
        primes.push(number);
        potential_primes.retain(|n| n % number != 0);
    }

    primes
}

fn primes_up_to_traditional(limit: u64) -> Vec<u64> {
    let mut primes = vec![];

    for number in 2..=limit {
        let mut prime = true;

        for n in &primes {
            if n > &sqrt_int(number) {
                break;
            } else if number % n == 0 {
                prime = false;
                break;
            }
        }

        if prime {
            primes.push(number);
        }
    }

    primes
}

fn sqrt_int(n: u64) -> u64 {
    for i in 0..=n {
        if i * i > n {
            return i - 1;
        }
    }

    unreachable!();
}

fn main() {
    let limit = match args().collect::<Vec<_>>().get(1) {
        Some(v) => match v.parse::<u64>() {
            Ok(v) => v,
            Err(_) => {
                println!("Couldn't parse input (did you provide a positive number?)");
                return;
            }
        }
        None => {
            println!("Please provide a number.");
            return;
        }
    };

    let mut stdout_handle = stdout();

    let before: Instant = Instant::now();
    eprintln!("Seive");
    for n in primes_up_to(limit) {
        stdout_handle.write_all(format!("{n}\n").as_bytes()).unwrap();
    }
    let after = Instant::now();
    eprintln!("Took {:?}", after - before);

    let before = Instant::now();
    eprintln!("No seive");
    for n in primes_up_to_traditional(limit) {
        stdout_handle.write_all(format!("{n}\n").as_bytes()).unwrap();
    }
    let after = Instant::now();
    eprintln!("Took {:?}", after - before);

    stdout_handle.flush().unwrap();
}
