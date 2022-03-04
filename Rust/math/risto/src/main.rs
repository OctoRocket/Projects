use std::io;

fn main() {
    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");
    let mut num = num.trim().parse::<i32>().expect("Failed to parse");
    let mut iter = 0;
    let mut neg = false;
    loop {
        if num == 0 {
            break;
        } else if num < 0 {
            neg = true;
            num *= -1;
        }
        let temp = num.to_string().chars().rev().collect::<String>();
        let mut rev_num = temp.parse::<i32>().expect("Failed to parse");
        if neg == true {
            rev_num *= -1;
            num *= -1;
        }
        println!("num: {}, rev_num: {}", num, rev_num);
        num = num - rev_num;
        iter += 1;
        neg = false;
    }
    println!("\n{}", iter);
}
