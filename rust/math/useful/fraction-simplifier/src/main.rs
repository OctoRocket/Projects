fn input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

// euclidean algormithm
fn gcd(mut n1: u32, mut n2: u32) -> u32 {
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

fn simplify(input: String) -> String {
    let nums: Vec<u32> = input.split("/").map(|f| f.parse::<u32>().unwrap()).collect();
    let gcd = gcd(nums[0], nums[1]);
    format!("{}/{}", nums[0]/gcd, nums[1]/gcd)
}

fn main() {
    let input = input();
    if {
        let check: Vec<&str> = input.split("/").collect();
        check[0].parse::<u32>().is_ok() && check[1].parse::<u32>().is_ok()
    } == false {
        println!("Not a fraction");
        return;
    }
    println!("{}", simplify(input));
}
