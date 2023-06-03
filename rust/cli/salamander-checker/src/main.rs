mod args;
mod functions;

use args::Args;
use clap::Parser;
use std::{
    thread,
    time::Duration,
};

fn main() {
    let args = Args::parse();
    let link = args.link;
    println!("Watching link: {}", link);

    loop {
        println!("Player count: {}", functions::get_player_count(&link).unwrap());

        thread::sleep(Duration::from_secs(15));
    }
}
