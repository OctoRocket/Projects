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

    if a.is_empty() {
        result.append(&mut b);
    } else {
        result.append(&mut a);
    }

    result
}

fn merge_sort<T: PartialOrd + Copy>(to_sort: Vec<T>) -> Vec<T> {
    if to_sort.len() == 1 {
        return to_sort;
    }

    let left  = to_sort[0..(to_sort.len() / 2)].to_vec();
    let right = to_sort[(to_sort.len() / 2)..(to_sort.len())].to_vec();

    merge(merge_sort(left), merge_sort(right))
}

fn main() {
    let unsorted = vec![1, 8, 7, 2, 4, 6];

    println!("{:?}", merge_sort(unsorted));
}
