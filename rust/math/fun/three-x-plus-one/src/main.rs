#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery)]

mod args;
mod functions;

use args::{ProgramMode, ProgramSubcommand};
use clap::Parser;
use functions::{auto, manual};

fn main() {
    let args = ProgramSubcommand::parse();

    match args.subcommand {
        ProgramMode::Manual(args) => manual(args.number, args.delay),
        ProgramMode::Auto(args) => auto(args.delay),
    }
}
