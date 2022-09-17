fn input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

// euclidean algormithm
fn gcd(mut n1: u64, mut n2: u64) -> u64 {
    loop {
        if n1 == 0 || n2 == 0 {
            if n1 > n2 {
                return n1;
            } else {
                return n2;
            }
        } else if n1 > n2 || n1 == n2 {
            n1 = n1 % n2;
        } else if n2 > n1 {
            n2 = n2 % n1;
        }
    };
}

fn main() {
    let input = input();
    if input.matches(".").count() != 1 {
        println!("Not a valid decimal");
        return;
    }
    // get the number of places after the decimal point (eg. 10.48 is 2) 
    let places_after_point = input.split(".").nth(1).unwrap().len() as u32;
    // process the input 
    let input = {
        let split = input.split(".").collect::<Vec<&str>>();
        let split = split[0].to_string() + split[1];
        split.parse::<u64>().unwrap()
    };
    let denominator = 10u64.pow(places_after_point);
    let gcd = gcd(input, denominator);
    println!("{}/{}", input/gcd, denominator/gcd);
}
