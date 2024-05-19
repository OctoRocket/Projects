fn fib(len: u128) -> Vec<u128> {
    let mut fib_list = vec![0, 1];
    for _ in 0..len {
        fib_list.push(fib_list[(fib_list.len() - 2)..fib_list.len()].iter().sum());
    }

    fib_list[0..(fib_list.len() - 2)].to_vec()
}

fn build_string(fib_numbers: &[u128]) -> String {
    fib_numbers
        .iter()
        .map(std::string::ToString::to_string)
        .collect::<Vec<String>>()
        .join(", ")
        .trim_end_matches(", ")
        .to_owned()
}

fn main() {
    println!("Enter the number of numbers of the fib seq that you want.");
    let count = {
        let mut buf = String::new();
        std::io::stdin()
            .read_line(&mut buf)
            .expect("Failed to read line.");
        buf.trim().parse::<u128>().expect("Not a valid number.")
    };

    println!("{}", build_string(&fib(count)));
}
