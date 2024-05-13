use std::env::args;

fn main() {
    let input = args()
        .nth(1)
        .expect("Please provide an input")
        .trim()
        .parse()
        .expect("Please provide a number");

    // verify that the input is positive
    if input < 1 {
        panic!("Please provide a positive number");
    }

    // get a range of numbers from 1 to n, then filter it so that only factors remain
    let range = (1..input).filter(|x| input % x == 0);
    let mut vec = Vec::new();

    // format it correctly
    for i in range {
        if input % i == 0 {
            vec.push((input / i).to_string() + " * " + &i.to_string());
        }
    }

    // remove half of the list, as half of it will just be the flipped version of the other half (eg. 9*2 and 2*9)
    let flip = usize::from(vec.len() % 2 == 1);
    if vec.len() != 2 {
        for _i in vec.len() / 2 + 1..vec.len() - flip {
            vec.pop();
        }
    }

    // print the result
    for i in vec {
        println!("{i}");
    }
}
