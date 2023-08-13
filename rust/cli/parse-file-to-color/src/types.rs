use colored::Styles;
use clap::Parser;

// Colored
pub struct Line {
    pub text: String,
    pub fg_color: Option<String>,
    pub bg_color: Option<String>,
    pub style: Option<Vec<Styles>>,
}

// Clap
#[derive(Parser)]
#[command(
    name = "parse file to color",
    author = "OctoRocket",
    about = "A simple tool to parse a file to color",
)]
pub struct Args {
    /// The files to parse.
    #[clap(short, long, default_value = "example.txt")]
    pub file: String,
}
