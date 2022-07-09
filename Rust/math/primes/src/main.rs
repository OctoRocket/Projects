macro_rules! input {
    () => {{
            let mut buf = String::new();
            std::io::stdin().read_line(&mut buf).unwrap();
            buf.trim().parse().unwrap()
        }};
}

fn primes_under(limit: u64) -> Vec<u64> {
    let mut primes = Vec::new();
    if limit > 2 {
        primes.push(2);
    } else {
        return vec![];
    }
    for i in 3..=limit {
        let mut is_prime = true;
        for j in &primes {
            if i%j == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime == true {
            primes.push(i);
        }
    }
    primes
}

fn main() {
    let limit = input!();
    for i in primes_under(limit) {
        println!("{}", i);
    }
}