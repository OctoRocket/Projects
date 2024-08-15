use std::{env, fmt::Debug, fs::File, io::Read, ptr, time::Instant};

fn shift_down<T>(list: &mut [T], from: usize, to: usize) {
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

fn merge_sort<T: Ord + Send + Debug>(list: &mut [T]) {
    if list.len() == 1 {
        return;
    }

    let (split_a, split_b) = list.split_at_mut(list.len() / 2);
    rayon::join(
        || merge_sort(split_a),
        || merge_sort(split_b),
    );

    merge_in_place(list, list.len() / 2);
}

fn merge_in_place<T: Ord>(list: &mut [T], midpoint: usize) {
    let mut right_index = midpoint;

    for left_index in 0..list.len() {
        if left_index >= right_index || right_index >= list.len() {
            break;
        }

        if list[right_index] < list[left_index] {
            shift_down(list, right_index, left_index);
            right_index += 1;
        }
    }
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

    let mut list = lines.lines()
        .filter_map(|l| l.parse::<i64>().ok())
        .collect::<Vec<_>>();

    println!("{list:?}: start");
    let time_before = Instant::now();
    merge_sort(&mut list);
    let time_after = Instant::now();
    println!("{list:?}: end");
    eprintln!("Took {:?}.", time_after - time_before);
}
