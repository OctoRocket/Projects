use integer_sqrt::IntegerSquareRoot;
use std::io;

// calculate perfect squares
fn p_squares(limit: u32) -> Vec<u32> {
    let mut i = 1;
    let mut vec: Vec<u32> = Vec::new();
    while i * i <= limit {
        vec.push(i * i);
        i += 1;
    }
    vec
}

fn main() {
    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");
    let num = num.trim().parse::<u32>().expect("Error NaN");
    let mut ps = p_squares(num);
    let mut cont = true;
    // sort perfect squares from biggest to smallest and remove the number one
    ps.reverse();
    ps.pop();
    // for every perfect square, if it devides with no remainder, then check if the number is a perfect square and format it accordingly
    for i in ps {
        if num % i == 0 {
            if num / i == 1 {
                cont = false;
                println!("\n{}", num.integer_sqrt());
                break;
            } else {
                cont = false;
                println!("\n{}√{}", &i.integer_sqrt(), num / i);
                break;
            }
        }
    }
    if cont {
        println!("\n√{}", num);
    }
}
