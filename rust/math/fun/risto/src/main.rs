use std::io::stdin;

// This is a function to flip a number
fn flip(input: i32) -> i32 {
    if input.is_negative() {
        return -input.abs().to_string().chars().rev().collect::<String>().parse::<i32>().unwrap();
    }
    input.to_string().chars().rev().collect::<String>().parse::<i32>().unwrap()
}

// This fuction contains the logic for the program
fn risto(mut input: i32) -> Vec<i32> {
    let mut number_list = Vec::new();
    while input != 0 || !number_list.contains(&input) {
        number_list.push(input);
        input -= flip(input);
    }
    number_list.pop();
    number_list
}

/* All that main does is handle the input from stdin
   and making the output of risto look nicer         */
fn main() {
    let input = {
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();
        buf.trim().parse().expect("Parse error")
    };
    let output = risto(input);
    println!("It took {} iterations to get to 0,", output.len());
    for i in output {
        println!("{} - {} = {}", i, flip(i), i - flip(i));
    }
}
