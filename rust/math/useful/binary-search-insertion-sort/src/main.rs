fn insert_sorted<T: PartialOrd>(list: &mut Vec<T>, to_insert: T) {
    let mut bounds = (0, list.len());
    let mut index = (bounds.0 + bounds.1) / 2;

    while bounds.1 - bounds.0 > 1 {
        if list[index] > to_insert {
            bounds.1 = index;
        } else if list[index] < to_insert {
            bounds.0 = index;
        } else {
            break;
        }

        index = (bounds.0 + bounds.1) / 2;
    }

    let mut offset = 0;
    if !list.is_empty() {
        offset = (to_insert > list[index]) as usize;
    }

    list.insert(index + offset, to_insert);
}

fn sort<T: PartialOrd>(list: &mut Vec<T>) {
    let mut sorted = Vec::new();

    while let Some(element) = list.pop() {
        insert_sorted(&mut sorted, element);
    }

    *list = sorted;
}

fn main() {
    let mut list = vec![4, 7, 5, 0, 3, 6, 1, 9, 1, 0];

    sort(&mut list);
    println!("{list:?}");
}
