fn elem<T>(val: T, list: Vec<T>) -> bool
where T: Eq, T: std::clone::Clone {
    if list.is_empty() {
        return false;
    }

    let first = &list[0];
    let rest = {
        let mut tmp_list = list.clone();
        tmp_list.remove(0);
        tmp_list
    };

    (first == &val) || (elem(val, rest))
}

fn main() {
    println!("{}", elem(3, (1..=10).collect()));
}
