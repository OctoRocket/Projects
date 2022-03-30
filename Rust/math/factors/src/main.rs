use std::io;

fn get_inp() -> i64 {
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).expect("Failed to read line");
    inp.trim().parse::<i64>().expect("Failed to parse input")
}

fn main() {
    print!("Input a number: ");
    let n = get_inp();
    let range = (1..n).filter(|x| n % x == 0);
    let mut vec = vec!["\n".to_string()];
    for i in range {
        if n % i == 0 {
            vec.push((n/i).to_string() + "*" + &i.to_string());
        }
    }
    if vec.len() != 2 {
        for _i in (vec.len() as f32/2.0).ceil() as u32 + 1..vec.len() as u32 {
            vec.pop();
        }
    }
    for i in vec {
        println!("{}", i);
    }
}
