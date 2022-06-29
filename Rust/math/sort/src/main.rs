use rand::{thread_rng, prelude::SliceRandom};

fn main() {
    // take input
    let mut cap = String::new();
    std::io::stdin().read_line(&mut cap).expect("Failed to read line");
    let cap: usize = cap.trim().parse().expect("Please type a number!");

    let mut array: Vec<u32> =  Vec::with_capacity(cap);
    for i in 1..=array.capacity() {
        array.push(i as u32);
    }    
    let mut rng = thread_rng();
    array.shuffle(&mut rng);

    // print array
    println!("{:?}", array);

    // sort the array using merge sort
    let mut new_array = Vec::new();
    for i in 0..array.len()-1 {
        if array[i] > array[i+1] {
            new_array.push(vec![array[i+1], array[i]]);
        } else {
            new_array.push(vec![array[i], array[i+1]]);
        }
    }

    // finish sorting with merge sort
    while new_array.len() > 1 {
        let mut buffer = Vec::new();
        for i in new_array.clone() {
            for j in i {
                buffer.push(j);
            }
            let mut new_buffer = Vec::new();
            while buffer.len() > 1 {
                if buffer[0] < buffer[1] {
                    new_buffer.push(buffer[0]);
                    buffer.remove(0);
                } else {
                    new_buffer.push(buffer[1]);
                    buffer.remove(1);
                }
            }
            new_buffer.push(buffer[0]);
            println!("{:?}", new_buffer);
        }
        break;
    }
}
