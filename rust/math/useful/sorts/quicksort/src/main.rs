#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery)]

use std::{env, fs::File, io::Read, time::Instant};

fn quicksort<T: Ord + Clone + std::fmt::Debug>(list: &mut Vec<T>) {
    fn worker<T: Ord + Clone + std::fmt::Debug>(list: &mut Vec<T>, bottom: usize, top: usize) {
        if top.saturating_sub(bottom) < 2 { return; }
        println!("{list:?} | ({bottom} - {})", top - 1);

        let mut pivot_index = (top - bottom) / 2 + bottom;
        list.swap(pivot_index,  top - 1);
        pivot_index = top - 1;
        let pivot = list[top - 1].clone();

        // Sort left and right into less than and greater than sections respectively
        let mut left_index = bottom;
        let mut walk_index = bottom + 1;
        while walk_index < top {
            println!("{list:?} | (left: {left_index} - walk: {walk_index})");
            if list[walk_index] < pivot {
                list.swap(left_index, walk_index);
                left_index += 1;
            }
            walk_index += 1;
        }

        // Move pivot to the left index if left_index is bigger than pivot
        if list[left_index] > pivot {
            list.swap(pivot_index, left_index);
            pivot_index = left_index;
        }

        println!("{list:?} | (pivot: {pivot_index} -> {pivot:?})\n");

        worker(list, bottom, pivot_index);
        worker(list, pivot_index + 1, top);
    }

    let length = list.len();

    worker(list, 0, length);
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
    quicksort(&mut list);
    let time_after = Instant::now();
    println!("{list:?}: end");
    eprintln!("Took {:?}.", time_after - time_before);
}
