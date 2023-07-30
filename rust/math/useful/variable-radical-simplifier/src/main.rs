use std::io::stdin;

fn perfects(limit: u32, degree: u32) -> Vec<u32> {
    let mut perfects = Vec::new();
    let mut x: u32 = 1;
    while x.pow(degree) <= limit {
        perfects.push(x.pow(degree));
        x += 1;
    }
    perfects
}

fn main() {
    println!("Input radical and degree in format (inside of radical):(degree)");
    let input: Vec<u32> = {
        let mut inp = String::new();
        stdin().read_line(&mut inp).unwrap();
        inp.trim().split(':').map(|f| f.parse().unwrap()).collect()
    };
    let mut perfects = perfects(input[0], input[1]);
    perfects.reverse();
    for i in perfects {
        if input[0] % i == 0 {
            if input[0] / i == 1 {
                println!("{}", i / input[1]);
                return;
            }
            println!("{}rad({})", i / input[1], input[0] / i);
            return;
        }
    }
}
