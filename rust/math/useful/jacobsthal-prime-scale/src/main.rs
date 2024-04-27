fn is_prime(number: u32) -> bool {
    if number <= 3 {
        return true;
    }

    (2..=((number as f64).sqrt().floor() as u32)).map(|e| number % e).reduce(|a, b| a.min(b)).unwrap() != 0
}

fn jacobsthal(limit: u32) -> Vec<u32> {
    let mut primes = vec![1];
    let mut sequence = vec![1, 1];

    while primes[primes.len() - 1] < limit {
        let length = sequence.len();
        let next = sequence[length - 1] + 2 * sequence[length - 2]; 
        sequence.push(next);

        if is_prime(next) {
            primes.push(next);
        }
    }

    let mut result = vec![1];
    for index in 1..primes.len() {
        let next = ((primes[index - 1] + 1)..primes[index]).rev();

        result.push(primes[index]);
        result.extend(next);
    }

    result.into_iter().filter(|x| *x <= limit).collect()
}

fn main() {
    println!("{:?}", jacobsthal(7));
}
