fn input() -> u64 {
    let mut inp = String::new();
    std::io::stdin().read_line(&mut inp).unwrap();
    inp.trim().parse().unwrap()
}

fn primes_under(limit: u64) -> Vec<u64> {
    let mut primes = Vec::new();
    // input 2 as the first prime, otherwise there are no primes
    if limit > 2 {
        primes.push(2);
    } else {
        return vec![];
    }
    // from 3 (first number above the first prime) to the limit
    for i in 3..=limit {
        let mut is_prime = true;
        // if the number is divisable a prime number before it, then it is not prime.
        for j in &primes {
            if i % j == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            primes.push(i);
        }
    }
    primes
}

fn main() {
    let limit = input();
    for i in primes_under(limit) {
        println!("{}", i);
    }
}