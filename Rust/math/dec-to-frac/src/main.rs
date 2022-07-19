fn solve(mut frac: String, num: f64) -> String {
    let ef = evaluate_faction(frac.clone());
    let split = get_parts(frac.clone());
    if ef > num {
        frac = split[0].to_string() + "/" + &(split[1]+1).to_string();
    } else {
        frac = (split[0] + 1).to_string() + "/" + &split[1].to_string();
    }
    frac
}

fn get_input() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf
}

fn evaluate_faction(frac: String) -> f64 {
    let frac_vec = get_parts(frac);
    frac_vec[0] as f64 / frac_vec[1] as f64 
}

fn get_parts(frac: String) -> Vec<u32> {
    frac.split("/").map(|s| s.parse().unwrap()).collect::<Vec<u32>>()
}

fn precise_solve(num: f64) -> String {
    let mut frac = String::from("1/1");
    while evaluate_faction(frac.clone()) != num {
        frac = solve(frac, num);
    }
    frac
}

fn depth_solve(num: f64, depth: u32) -> String {
    let mut frac = String::from("1/1");
    for _i in 0..depth {
        frac = solve(frac, num);
        if evaluate_faction(frac.clone()) == num {
            break;
        }
    }
    frac
}

fn main() {
    let inp = get_input();
    let inp = inp.trim();
    let inp = inp.split(" ").collect::<Vec<&str>>();
    if inp.len() == 1 {
        println!("{}", precise_solve(inp[0].parse().unwrap()));
    } else if inp.len() == 2 {
        println!("{}", depth_solve(inp[0].parse().unwrap(), inp[1].parse().unwrap()))
    }
}
