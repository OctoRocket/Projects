fn main() {
    print!("Input a number: ");
    let n = match std::env::args().collect::<Vec<String>>()[1].parse() {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to parse, is your input a positve number? ({})", e);
            return;
        }
    };
    // get a range of numbers from 1 to n, then filter it so that only factors remain
    let range = (1..n).filter(|x| n % x == 0);
    let mut vec = vec!["\n".to_string()];
    // format it correctly
    for i in range {
        if n % i == 0 {
            vec.push((n/i).to_string() + "*" + &i.to_string());
        }
    }
    // remove half of the list, as half of it will just be the flipped version of the other half (eg. 9*2 and 2*9)
    let flip = usize::from(vec.len() % 2 == 1);
    if vec.len() != 2 {
        for _i in vec.len() / 2 + 1..vec.len() - flip {
            vec.pop();
        }
    }
    // print
    for i in vec {
        println!("{}", i);
    }
}
