fn main() {
    let mut tup = (0u128, 1u128);
    loop {
        println!("{}", tup.1);
        if tup.0.checked_add(tup.1).is_none() {
            break;
        }
        tup = (tup.1, tup.0+tup.1);
    }
}
