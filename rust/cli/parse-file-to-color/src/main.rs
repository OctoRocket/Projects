mod types;
mod functions;

use functions::{
    read_file,
    parse,
    str_to_rbg,
};
use colored::{
    Color,
    Colorize,
    Styles,
};
use clap::Parser;
use anyhow::Result;

fn main() -> Result<()>{
    let args = types::Args::parse();
    // read file to vector of lines
    let lines = parse(read_file(&args.file)?);
    for line in lines {
        let mut colored_line;
        if let Some(fg_color) = line.fg_color {
            if fg_color.is_empty() {
                colored_line = line.text.normal();
            } else if let Ok(color) = fg_color.replace('_', " ").parse::<Color>() {
                colored_line = line.text.color(color);
            } else if let Ok(color) = str_to_rbg(fg_color.as_str()) {
                colored_line = line.text.truecolor(color.0, color.1, color.2);
            } else {
                panic!("Invalid color, {fg_color}");
            }
        } else {
            colored_line = line.text.normal();
        }
        if let Some(bg_color) = line.bg_color {
            if bg_color.is_empty() {
                colored_line = colored_line.on_black();
            } else if let Ok(color) = bg_color.replace('_', " ").parse::<Color>() {
                colored_line = colored_line.on_color(color);
            } else if let Ok(color) = str_to_rbg(bg_color.as_str()) {
                colored_line = colored_line.on_truecolor(color.0, color.1, color.2);
            } else {
                panic!("Invalid color, {bg_color}");
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
                    Styles::Clear => colored_line = colored_line.clear(),
                }
            }
        }
        println!("{colored_line}");
    }
    Ok(())
}
