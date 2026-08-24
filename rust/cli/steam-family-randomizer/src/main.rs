#![warn(clippy::pedantic, clippy::nursery)]

mod args;
mod parser;

use args::Args;

use clap::Parser;
use anyhow::Result;
use reqwest::Client;
use std::{fs::File, io::{self, Read}};
use rand::prelude::*;

use crate::parser::{all_ids, title};

macro_rules! time {
    ($code:expr) => {{
        let prev = ::std::time::Instant::now();
        let res = $code;
        let now = ::std::time::Instant::now();
        eprintln!("Took {:?}", now - prev);
        res
    }};
}

const STEAM_URL: &str = "https://store.steampowered.com/app/";

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let mut rng = rand::rng();

    let client = Client::new();

    let mut file = File::open(args.file).unwrap();
    let mut buf = String::new();
    eprintln!("Reading file...");
    time!(file.read_to_string(&mut buf)?);
    eprintln!("Extracting IDs...");
    let mut ids = time!(all_ids(&buf));
    ids.sort_unstable();
    ids.dedup();

    eprintln!("Fetching game names...");

    let input = io::stdin();
    let mut all_game_names = ids;
    loop {
        let mut buf = String::new();
        input.read_line(&mut buf)?;
        all_game_names.shuffle(&mut rng);
        let name = get_game_name(&all_game_names[0], &client).await?;
    }

    // let har_file = File::open(args.file)?;
    // let parsed_har = parse_har(har_file)?;
    // let steam_game_ids = get_steam_game_ids(parsed_har);

    // Ok(())
}

async fn get_game_name(id: &str, client: &Client) -> Result<String> {
    let response = client.get(STEAM_URL.to_string() + id).send().await?;
    let title = dbg!(title(&response.text().await?));

    todo!()
}
