fn fib_increment(array: &[u128; 2]) -> Option<[u128; 2]> {
    Some([array[1], array[0].checked_add(array[1])?])
}

fn main() {
    let mut array = [0, 1];
    loop {
        println!("{}", array[0]);
        array = match fib_increment(&array) {
            Some(value) => value,
            None => break,
        };
    }
    println!("Done!");
}
