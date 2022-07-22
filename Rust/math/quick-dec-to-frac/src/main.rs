fn input() -> f64 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<f64>().unwrap()
}

fn gcd(mut n1: f64, mut n2: f64) -> f64 {
    loop {
        if n1 == 0.0 || n2 == 0.0 {
            if n1 > n2 {
                return n1;
            } else {
                return n2;
            }
        } else if n1 > n2 || n1 == n2 {
            n1 = n1 % n2;
        } else if n2 > n1 {
            n2 = n2 % n1;
        }
    };
}

fn main() {
    let mut input = input();
    let mut denominator = 1f64;
    while input.fract() != 0.0 {
        input *= 10.0;
        denominator *= 10.0;
    }
    let gcd = gcd(input, denominator);
    println!("{}/{}", input/gcd, denominator/gcd);
}
