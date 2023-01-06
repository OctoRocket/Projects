use std::io;

fn main() {
    // Prepare variables
    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");
    let mut num = num.trim().parse::<i32>().expect("Failed to parse");
    let mut iter = 0;
    let mut vec:Vec<i32> = Vec::new();
    loop {
        // Target is to reach 0
        if num == 0 {
            break;
        }
        // convert the int into a string then reverse it
        let mut temp = num.to_string().chars().rev().collect::<String>();
        // fix negitive issues
        if num < 0 {
            temp.pop();
            temp.insert(0, '-');
        }
        // parse string into an int again
        let rev_num = temp.parse::<i32>().expect("Failed to parse");
        // if it was negitive, return the negitive symbols
        println!("num: {}, rev_num: {}", num, rev_num);
        num -= rev_num;
        iter += 1;
        // detect if there is a loop
        for i in &mut vec {
            if num == *i {
                println!("Loop detected at iteration {}! Number: {}", iter, num);
                return;
            }
        }
        vec.push(num);
    }
    // print out how many iterations it took to get to 0
    println!("\n{}", iter);
}
