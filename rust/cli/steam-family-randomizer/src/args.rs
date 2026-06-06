use std::path::PathBuf;
use clap::Parser;

/// A utility to randomly choose a steam game from an entire steam library
#[derive(Parser, Debug)]
pub struct Args {
    /// Path to HAR file containing steam IDs
    #[arg(short, long)]
    pub file: PathBuf,

    /// Whether to list all detected games instead of picking a random one.
    #[arg(short, long, default_value_t = false)]
    pub list: bool,
}
