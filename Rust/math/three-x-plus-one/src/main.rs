use std::{thread, time, io};

fn sleep(time: u64) {
    thread::sleep(time::Duration::from_millis(time));
}

fn automatic_input(time: u64) {
    let mut b_num: u128 = 1;
    let mut r_num: u128 = 1;
    let mut iter: u32 = 0;
    loop {
        if r_num == 1 {
            println!("Done after {} iterations to get to 1, number was {}", iter, b_num);
            b_num += 1;
            r_num = b_num;
            // r_num = b_num.clone()
            iter = 0;
            sleep(time)
        } else if r_num % 2 == 0 {
            r_num = r_num/2;
            iter += 1;
        } else {
            r_num = r_num * 3 + 1;
            iter += 1;
        }
    }
}

fn main() {
    let mut t_inp = String::new();
    io::stdin()
        .read_line(&mut t_inp)
        .expect("Failed to read line");
    let t_inp = t_inp.trim().parse::<u64>().expect("NaN");
    automatic_input(t_inp)
}
