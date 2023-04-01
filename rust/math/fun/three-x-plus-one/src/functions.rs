pub fn manual(mut number: u64, delay: u64) {
    let mut iterations = 0;

    while number != 1 {
        println!("{}", number);
        number = three_x_plus_one(number);
        iterations += 1;
        std::thread::sleep(std::time::Duration::from_millis(delay));
    }

    println!("It took {} iterations", iterations);
}

pub fn auto(delay: u64) {
    let mut number = 1;
    let mut running_value = number;
    let mut iterations = 0;

    loop {
        while running_value != 1 {
            running_value = three_x_plus_one(running_value);
            iterations += 1;
        }
        println!("{}: {}", number, iterations);
        
        number += 1;
        running_value = number;
        iterations = 0;
        
        std::thread::sleep(std::time::Duration::from_millis(delay));
    }
}

fn three_x_plus_one(number: u64) -> u64 {
    if number % 2 == 0 {
        number / 2
    } else {
        (number * 3) + 1
    }
}
