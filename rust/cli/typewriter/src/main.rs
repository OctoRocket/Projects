#![deny(clippy::all)]
#![warn(
    clippy::pedantic,
    clippy::nursery,
)]

mod args;

use std::{
    io::{
        BufRead,
        Write,
        stdin,
        stdout,
    },
    thread::sleep,
    time::Duration,
};
use args::Args;
use anyhow::{
    Result,
    Ok,
};
use rand::{
    Rng,
    thread_rng,
};
use clap::Parser;

fn duration(args: &Args) -> Result<u64> {
    
    if let Some(random) = &args.random {
        return Ok(thread_rng().gen_range(random[0]..=random[1]));
    }

    Ok(args.delay)
}

fn main() -> Result<()>{
    let args = Args::parse();
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
        sleep(Duration::from_millis(duration(&args)?));
        stdout().flush()?;
    }

    Ok(())
}