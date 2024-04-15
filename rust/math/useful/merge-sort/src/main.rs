fn merge<T: PartialOrd + Copy>(mut a: Vec<T>, mut b: Vec<T>) -> Vec<T> {
    let mut result = vec![];

    while !a.is_empty() && !b.is_empty() {
        if a[0] < b[0] {
            result.push(a[0]);
            a.remove(0);
        } else {
            result.push(b[0]);
            b.remove(0);
        }
    }

    // One list in guaranteed to be empty, appending both will only append leftovers
    result.append(&mut a);
    result.append(&mut b);
    result
}

fn merge_sort<T: PartialOrd + Copy>(to_sort: Vec<T>) -> Vec<T> {
    if to_sort.len() == 1 {
        return to_sort;
    }

    let split = to_sort.split_at(to_sort.len() / 2);

    merge(merge_sort(split.0.to_vec()), merge_sort(split.1.to_vec()))
}

fn main() {
    let unsorted = vec![1, 8, 7, 2, 4, 6];

    println!("{:?}", merge_sort(unsorted));
}
