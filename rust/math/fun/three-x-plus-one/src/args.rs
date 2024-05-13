use clap::{
    Args,
    Subcommand,
    Parser,
};

#[derive(Debug, Parser)]
#[clap(author, about, version)]
pub struct ProgramSubcommand {
    #[clap(subcommand)]
    pub subcommand: ProgramMode,
}

#[derive(Debug, Subcommand)]
pub enum ProgramMode {
    /// Input a number and an optional delay and get the 3x+1 sequence
    Manual(Manual),

    // Input a delay and get the 3x+1 sequence numbers counting up from 1
    Auto(Auto),
}

#[derive(Debug, Args)]
pub struct Manual {
    /// The number to start the sequence from
    pub number: u64,

    /// The delay between each number in milliseconds
    #[clap(short, long, default_value = "250")]
    pub delay: u64,
}

#[derive(Debug, Args)]
pub struct Auto {
    /// The delay between each number in milliseconds
    #[clap(short, long, default_value = "250")]
    pub delay: u64,
}
