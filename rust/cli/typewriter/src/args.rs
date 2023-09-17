use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg(short, long, default_value_t = 100)]
    pub delay: u64,

    #[arg(short, long, num_args = 2)]
    pub random: Option<Vec<u64>>,
}
