use std::env::args;

const fn int_sqrt(num: usize) -> usize {
    let mut root = 1;

    while (root + 1) * (root + 1) <= num {
        root += 1;
    }

    root
}

fn get_factors(num: usize) -> impl Iterator<Item = (usize, usize)> {
    (1..=int_sqrt(num))
        .filter(move |n| num % n == 0)
        .map(move |n| (n, num / n))
}

fn main() {
    let input = args()
        .nth(1)
        .expect("Please provide an input")
        .trim()
        .parse()
        .expect("Please provide a number");

    get_factors(input).for_each(|p| println!("{} x {}", p.0, p.1));
}
