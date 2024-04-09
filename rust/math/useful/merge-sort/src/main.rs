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

fn main() {
    let a = vec![1, 3, 4];
    let b = vec![2, 4, 5];

    println!("A: {:?}\nB: {:?}", a, b);
    println!("Merged: {:?}", merge(a, b));
}
