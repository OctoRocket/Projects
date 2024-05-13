#![deny(clippy::all)]
#![warn(
    clippy::pedantic,
    clippy::nursery,
)]

mod args;
mod functions;

use clap::Parser;
use functions::{
    manual,
    auto,
};
use args::{
    ProgramSubcommand,
    ProgramMode,
};

fn main() {
    let args = ProgramSubcommand::parse();

    match args.subcommand {
        ProgramMode::Manual(args) => manual(args.number, args.delay),
        ProgramMode::Auto(args) => auto(args.delay),
    }
}
