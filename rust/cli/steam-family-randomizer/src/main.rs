#![warn(clippy::pedantic, clippy::nursery)]

mod args;
mod parser;

use args::Args;

use clap::Parser;
use anyhow::Result;
use std::{fs::File, io::Read, path::PathBuf};

use crate::parser::{all_ids, steam_id, first_id};

fn main() -> Result<()> {
    let args = Args::parse();

    let mut file = File::open(PathBuf::from("example2.har")).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf);
    let mut v = all_ids(&buf);
    v.sort_unstable();
    v.dedup();
    eprintln!("{v:?}, {}", v.len());

    // let har_file = File::open(args.file)?;
    // let parsed_har = parse_har(har_file)?;
    // let steam_game_ids = get_steam_game_ids(parsed_har);

    Ok(())
}
