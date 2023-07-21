fn main() {
    println!("{}", match std::env::args().nth(1){ Some(s) => s.len(), None => 0 });
}
