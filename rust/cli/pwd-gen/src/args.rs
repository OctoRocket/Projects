/// This file contains the logic for handling the command line arguments.

use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "pwd-gen",
    author = "OctoRocket",
    about = "A simple password generator written in Rust."
)]
pub struct Args {
    /// Which characters to generate with.
    /// l = lowercase, u = uppercase, d = digits, s = symbols.
    #[arg(short, long, default_value = "luds")]
    pub chars: String,

    /// The length of the password.
    #[arg(short, long, default_value = "16")]
    pub length: usize,
}
