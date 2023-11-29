#![deny(clippy::all)]
#![warn(
    clippy::pedantic,
    clippy::nursery,
)]

use std::{
    fs::File,
    path::PathBuf,
    io::Read,
    env::args,
};
use anyhow::{
    Result,
    Error,
};
use colored::{
    Color,
    Colorize,
};
use regex::Regex;
use thiserror::Error;

#[derive(Error, Debug)]
enum ProgramError {
    #[error("no path provided")]
    PathError,

    #[error("missing color tag")]
    ColorTagError,

    #[error("text malformation error")]
    TextError,

    #[error("unindentifiable color tag")]
    ConversionError,
}

#[derive(Debug)]
struct  Text {
    color_tag: Color,
    content: String
}

impl Text {
    fn new(color_tag: Color, content: String) -> Self {
        Text {
            color_tag,
            content,
        }
    }
}

fn to_color(text: &str) -> Result<Color> {
    let color = match text {
        r"{c0}" => Ok(Color::White),
        r"{c1}" => Ok(Color::Red),
        r"{c2}" => Ok(Color::Green),
        r"{c3}" => Ok(Color::Blue),
        r"{c4}" => Ok(Color::Yellow),
        r"{c5}" => Ok(Color::Cyan),
        r"{c6}" => Ok(Color::Magenta),
        r"{c7}" => Ok(Color::Black),
        _ => Err(Error::new(ProgramError::ConversionError)),
    };
    
    color
}

fn split_text(text: String) -> Result<Text> {
    let re = Regex::new(r"\{.*?\}")?;
    let color_tag = re.find(&text).ok_or(ProgramError::ColorTagError)?.as_str();
    let content = re.split(&text).nth(1).ok_or(ProgramError::TextError)?;

    Ok(Text::new(to_color(color_tag)?, content.to_string()))
}

fn parse_file(mut file: File) -> Result<Vec<Text>> {
    let re = Regex::new(r"\{[^{]*")?;
    let mut file_buf = String::new();
    file.read_to_string(&mut file_buf)?;
    let prepended_file = String::from(r"{c0}") + &file_buf;

    let segments = re.find_iter(&prepended_file).map(|m| m.as_str().to_string());
    let mut texts = Vec::new();

    for s in segments {
        texts.push(split_text(s)?)
    }

    Ok(texts)
}

fn main() -> Result<()> {
    let path = args().nth(1);
    if path.is_none() {
        return Err(Error::new(ProgramError::PathError));
    } 
    let path = path.unwrap();

    let file = File::open(PathBuf::from(path))?;
    
    for segment in parse_file(file)? {
        print!("{}", segment.content.as_str().color(segment.color_tag))
    }

    Ok(())
}
