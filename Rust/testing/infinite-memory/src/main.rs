use std::vec;

fn main() {
    let mut vec = vec![2147483647];
    loop {
        vec.push(2147483647);
    }
}
