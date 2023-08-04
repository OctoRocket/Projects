/// This file contains the argument handling logic

use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "salachecker",
    author = "OctoRocket",
    version = "0.1.0",
    about = "A simple tui for watching the player count of a SS14 server (Salamander specifically)."
)]
pub struct Args {
    /// Link to watch the player count of.
    #[arg(short, long, default_value = "https://lizard.spacestation14.io/salamander/status")]
    pub link: String,
}
