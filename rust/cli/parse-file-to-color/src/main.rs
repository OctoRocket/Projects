mod types;
mod functions;

use functions::{
    read_file,
    parse,
    str_to_rbg,
};
use colored::{
    Colorize,
    Styles,
};
use clap::Parser;
use anyhow::Result;

fn main() -> Result<()>{
    let args = types::Args::parse();
    // read file to vector of lines
    let lines = parse(read_file(&args.file)?)?;
    for line in lines {
        let mut colored_line;
        if let Some(fg_color) = line.fg_color {
            if fg_color == "" {
                colored_line = line.text.normal();
            } else {
                match fg_color.as_str() {
                    "black" => colored_line = line.text.black(),
                    "red" => colored_line = line.text.red(),
                    "green" => colored_line = line.text.green(),
                    "yellow" => colored_line = line.text.yellow(),
                    "blue" => colored_line = line.text.blue(),
                    "magenta" => colored_line = line.text.magenta(),
                    "cyan" => colored_line = line.text.cyan(),
                    "white" => colored_line = line.text.white(),
                    "bright_black" => colored_line = line.text.bright_black(),
                    "bright_red" => colored_line = line.text.bright_red(),
                    "bright_green" => colored_line = line.text.bright_green(),
                    "bright_yellow" => colored_line = line.text.bright_yellow(),
                    "bright_blue" => colored_line = line.text.bright_blue(),
                    "bright_magenta" => colored_line = line.text.bright_magenta(),
                    "bright_cyan" => colored_line = line.text.bright_cyan(),
                    "bright_white" => colored_line = line.text.bright_white(),
                    _ => {
                        if let Ok(color) = str_to_rbg(fg_color.as_str()) {
                            colored_line = line.text.truecolor(color.0, color.1, color.2);
                        } else {
                            panic!("Invalid color, {}", fg_color);
                        }
                    }
                }
            }
        } else {
            colored_line = line.text.normal();
        }
        if let Some(bg_color) = line.bg_color {
            if bg_color == "" {
                colored_line = colored_line.on_black();
            } else {
                match bg_color.as_str() {
                    "black" => colored_line = colored_line.on_black(),
                    "red" => colored_line = colored_line.on_red(),
                    "green" => colored_line = colored_line.on_green(),
                    "yellow" => colored_line = colored_line.on_yellow(),
                    "blue" => colored_line = colored_line.on_blue(),
                    "magenta" => colored_line = colored_line.on_magenta(),
                    "cyan" => colored_line = colored_line.on_cyan(),
                    "white" => colored_line = colored_line.on_white(),
                    "bright_black" => colored_line = colored_line.on_bright_black(),
                    "bright_red" => colored_line = colored_line.on_bright_red(),
                    "bright_green" => colored_line = colored_line.on_bright_green(),
                    "bright_yellow" => colored_line = colored_line.on_bright_yellow(),
                    "bright_blue" => colored_line = colored_line.on_bright_blue(),
                    "bright_magenta" => colored_line = colored_line.on_bright_magenta(),
                    "bright_cyan" => colored_line = colored_line.on_bright_cyan(),
                    "bright_white" => colored_line = colored_line.on_bright_white(),
                    _ => {
                        if let Ok(color) = str_to_rbg(&bg_color) {
                            colored_line = colored_line.on_truecolor(color.0, color.1, color.2);
                        } else {
                            panic!("Invalid color, {}", bg_color)
                        }
                    }
                }
            }
        }
        if let Some(styles) = line.style {
            for style in styles {
                match style {
                    Styles::Bold => colored_line = colored_line.bold(),
                    Styles::Italic => colored_line = colored_line.italic(),
                    Styles::Underline => colored_line = colored_line.underline(),
                    Styles::Dimmed => colored_line = colored_line.dimmed(),
                    Styles::Reversed => colored_line = colored_line.reversed(),
                    Styles::Blink => colored_line = colored_line.blink(),
                    Styles::Hidden => colored_line = colored_line.hidden(),
                    Styles::Strikethrough => colored_line = colored_line.strikethrough(),
                    _ => panic!("Invalid style, {:?}", style)
                }
            }
        }
        println!("{}", colored_line);
    }
    Ok(())
}
