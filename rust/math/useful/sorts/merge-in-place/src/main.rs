fn merge_in_place<T: Ord>(
    list: &mut Vec<T>,
    list_a_start: usize,
    list_b_start: usize,
    total_list_len: usize,
) {
    let mut index_a = list_a_start;
    let mut list_b_max = list_b_start;
    let mut index_b = list_b_start;

    loop {
        if index_a == list_b_max || index_b == total_list_len {
            break;
        }

        if list[index_a] > list[index_b] {
            let element = list.remove(index_b);
            list.insert(index_a, element);
            index_a += 1;
            index_b += 1;
            list_b_max += 1;
        } else {
            index_a += 1;
        }
    }
}

fn mergesort<T: Ord>(list: &mut Vec<T>) {
    fn worker<T: Ord>(list: &mut Vec<T>, sublist_start: usize, sublist_end: usize) {
        if sublist_start == sublist_end || sublist_start == sublist_end - 1 {
            return;
        }

        let midpoint = (sublist_end - sublist_start) / 2 + sublist_start;
        worker(list, sublist_start, midpoint);
        worker(list, midpoint, sublist_end);
        merge_in_place(list, sublist_start, midpoint, sublist_end)
    }

    worker(list, 0, list.len());
}

fn main() {
    let mut list = vec![6, 2, 6, 7, 9, 0, 2, 4, 0, 1, 2, 6, 4, 3, 6, 2];
    println!("{list:?}: start");
    mergesort(&mut list);
    println!("{list:?}: end");
}
