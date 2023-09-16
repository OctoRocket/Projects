#![deny(clippy::all)]
#![warn(
    clippy::pedantic,
    clippy::nursery,
)]

use std::{
    io::{
        BufRead,
        Write,
        stdin,
        stdout,
    },
    thread::sleep,
    time::Duration,
    env::{
        Args,
        args,
    },
};
use anyhow::Result;
use rand::{
    Rng,
    thread_rng,
};

fn duration(args: Args) -> Result<u64> {
    let mut args = args;

    if let Some(second_arg) = args.nth(2) {
        let first_arg = args
            .nth(1)
            .unwrap()
            .parse::<u64>()?;
        let second_arg = second_arg.parse::<u64>()?;
        Ok(thread_rng().gen_range(first_arg..=second_arg))
    } else if let Some(first_arg) = args.nth(1) {
        Ok(first_arg.parse::<u64>()?)
    } else {
        Ok(25)
    }
}

fn main() -> Result<()>{
    let mut chars = Vec::new();

    let lines = stdin().lock().lines();
    for line in lines {
        line?
            .chars()
            .for_each(
                |c|
                chars.push(c)
            );
        chars.push('\n');
    }

    for character in chars {
        print!("{character}");
        sleep(Duration::from_millis(duration(args())?));
        stdout().flush()?;
    }

    Ok(())
}