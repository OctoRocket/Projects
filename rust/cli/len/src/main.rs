fn main() {
    println!("{}", std::env::args().nth(1).unwrap_or_default().len());
}
