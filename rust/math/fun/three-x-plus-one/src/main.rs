mod args;
mod functions;

use clap::Parser;
use functions::{
    manual,
    auto,
};
use args::{
    ProgramArgs,
    ProgramSubcommand,
};

fn main() {
    let args = ProgramArgs::parse();
    match args.subcommand {
        ProgramSubcommand::Manual(args) => {
            manual(args.number, args.delay)
        }
        ProgramSubcommand::Auto(args) => {
            auto(args.delay)
        }
    }
}
