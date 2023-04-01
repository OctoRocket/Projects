use clap::{
    Args,
    Subcommand,
    Parser,
};

#[derive(Debug, Parser)]
#[clap(author, about, version)]
pub struct ProgramArgs {
    #[clap(subcommand)]
    pub subcommand: ProgramSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum ProgramSubcommand {
    /// Input a number and an optional delay and get the 3x+1 sequence
    Manual(ManualArgs),

    // Input a delay and get the 3x+1 sequence numbers counting up from 1
    Auto(AutoArgs),
}

#[derive(Debug, Args)]
pub struct ManualArgs {
    /// The number to start the sequence from
    pub number: u64,

    /// The delay between each number in milliseconds
    #[clap(short, long, default_value = "250")]
    pub delay: u64,
}

#[derive(Debug, Args)]
pub struct AutoArgs {
    /// The delay between each number in milliseconds
    #[clap(short, long, default_value = "250")]
    pub delay: u64,
}
