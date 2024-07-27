use std::{env, fs::File, io::Read, time::Instant};

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
    [result, a, b].concat()
}

fn merge_sort<T: PartialOrd + Copy>(to_sort: Vec<T>) -> Vec<T> {
    if to_sort.len() == 1 {
        return to_sort;
    }

    let split = to_sort.split_at(to_sort.len() / 2);

    merge(merge_sort(split.0.to_vec()), merge_sort(split.1.to_vec()))
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
    list = merge_sort(list);
    let time_after = Instant::now();
    println!("{list:?}: end");
    eprintln!("Took {:?}.", time_after - time_before);
}
