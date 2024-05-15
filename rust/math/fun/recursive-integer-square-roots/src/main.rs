fn recursive_int_sqrt(n: u64) -> u64 {
    fn internal_sqrt(n: u64, l: u64) -> u64 {
        if n * n <= l {
            n
        } else {
            internal_sqrt(n - 1, l)
        }
    }

    internal_sqrt(n, n)
}

fn bottom_up_recursive_int_sqrt(n: u64) -> u64 {
    fn internal_sqrt(n: u64, l: u64) -> u64 {
        if (n + 1) * (n + 1) > l {
            n
        } else {
            internal_sqrt(n + 1, l)
        }
    }

    internal_sqrt(1, n)
}

fn main() {
    println!("Top down:");
    for num in 1..=16 {
        println!("{}: {}", num, recursive_int_sqrt(num));
    }

    println!("\nBottom up:");
    for num in 1..=16 {
        println!("{}: {}", num, bottom_up_recursive_int_sqrt(num));
    }
}
