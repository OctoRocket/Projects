use std::{io, thread, time};

fn sleep(time: u64) {
    thread::sleep(time::Duration::from_millis(time));
}

fn a_inp(time: u64) {
    let mut b_num: u128 = 1;
    let mut r_num: u128 = 1;
    let mut iter: u32 = 0;
    loop {
        if r_num == 1 {
            println!(
                "Done after {} iterations to get to 1, number was {}",
                iter, b_num
            );
            b_num += 1;
            r_num = b_num;
            // r_num = b_num.clone()
            iter = 0;
            sleep(time)
        } else if r_num % 2 == 0 {
            r_num = r_num / 2;
            iter += 1;
        } else {
            r_num = r_num * 3 + 1;
            iter += 1;
        }
    }
}

fn m_inp(b_num: u128, delay: u64) -> u32 {
    let mut r_num: u128 = b_num;
    let mut iter: u32 = 0;
    loop {
        if r_num == 1 {
            return iter;
        } else if r_num % 2 == 0 {
            r_num = r_num / 2;
            iter += 1;
            println!("Iteration {}, running value is {}. Current number: {}.", iter, r_num, b_num);
            sleep(delay)
        } else {
            r_num = r_num * 3 + 1;
            iter += 1;
            sleep(delay)
        }
    }
}

fn main() {
    println!("1 for automatic input, 2 for manual input, 0 for exit: ");
    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read line");
    let inp = inp.trim().parse::<u16>().expect("Failed to parse");
    if inp == 1 {
        let mut inp = String::new();
        println!("Enter delay between outputs (MS): ");
        io::stdin()
            .read_line(&mut inp)
            .expect("Failed to read line");
        let inp = inp.trim().parse::<u64>().expect("Failed to parse");
        a_inp(inp);
    } else if inp == 2 {
        let mut inp = String::new();
        println!("How long of a delay between iterations do you want? (MS)");
        io::stdin()
            .read_line(&mut inp)
            .expect("Failed to read line");
        let delay = inp.trim().parse::<u64>().expect("Failed to parse");
        loop {
            let mut inp = String::new();
            println!("What number do you want to calculate? (0 to exit)");
            io::stdin()
                .read_line(&mut inp)
                .expect("Failed to read line");
            let inp = inp.trim().parse::<u128>().expect("Failed to parse");
            if inp == 0 {
                break();
            } else {
                println!("\nIt took {} iterations.\n", m_inp(inp, delay));
            }
        }
    }
}
