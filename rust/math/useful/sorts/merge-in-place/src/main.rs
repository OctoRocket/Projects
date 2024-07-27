use std::{env, fs::File, io::Read, ptr, time::Instant};

fn shift_down<T>(list: &mut Vec<T>, from: usize, to: usize) {
    if from >= list.len() {
        panic!("Tried to move from out of bounds index! (Index was {from} but list length is {})", list.len());
    }
    if to > list.len() {
        panic!("Tried to move to out of index! (Index was {to} but list length is {})", list.len());
    }
    if to > from {
        print!("Tried to move from smaller index to larger index! (From index {from} to index {to})");
    }

    unsafe {
        let from_ptr = list.as_mut_ptr().add(from);
        let to_ptr = list.as_mut_ptr().add(to);
        let moved = from_ptr.read();
        ptr::copy(to_ptr, to_ptr.add(1), from - to);
        ptr::write(to_ptr, moved);
    }
}

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
            shift_down(list, index_b, index_a);
            // let element = list.remove(index_b);
            // list.insert(index_a, element);
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
    let Some(file_name) = env::args().nth(1) else {
        eprintln!("Please provide a file of newline seperated numbers to sort.");
        return;
    };
    let mut file = match File::open(&file_name) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open file {file_name}. ({e})");
            return;
        }
    };
    let mut lines = String::new();
    if let Ok(s) = file.read_to_string(&mut lines) {
        s
    } else {
        eprintln!("File isn't valid UTF-8! Are you sure this is a text file?");
        return;
    };

    let mut list = lines.lines().filter_map(|l| l.parse::<i64>().ok()).collect();

    println!("{list:?}: start");
    let time_before = Instant::now();
    mergesort(&mut list);
    let time_after = Instant::now();
    println!("{list:?}: end");
    eprintln!("Took {:?}.", time_after - time_before);
}
