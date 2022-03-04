use std::io;

fn main() {
    // Prepare variables
    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");
    let mut num = num.trim().parse::<i32>().expect("Failed to parse");
    let mut iter = 0;
    let mut neg = false;
    loop {
        // Target is to reach 0
        if num == 0 {
            break;
        } else if num < 0 { // We don't want srings like 9- instead of -9 so we need to remove the negitive
            neg = true;
            num *= -1;
        }
        // convert the int into a string then reverse it
        let temp = num.to_string().chars().rev().collect::<String>();
        // parse string into an int again
        let mut rev_num = temp.parse::<i32>().expect("Failed to parse");
        // if it was negitive, return the negitive symbols
        if neg == true {
            rev_num *= -1;
            num *= -1;
        }
        println!("num: {}, rev_num: {}", num, rev_num);
        num = num - rev_num;
        iter += 1;
        neg = false;
    }
    // print out how many iterations it took to get to 0
    println!("\n{}", iter);
}
