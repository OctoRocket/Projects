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
    thread::sleep,
    time::Duration,
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
    Path,

    #[error("missing color tag")]
    ColorTag,

    #[error("text malformation error")]
    Text,

    #[error("unindentifiable color tag: {0}")]
    Conversion(String),
}

#[derive(Debug)]
struct  Text {
    color_tag: Color,
    content: String
}

fn to_color(text: &str) -> Result<Color> {
    match text {
        r"{c0}" => Ok(Color::White),
        r"{c1}" => Ok(Color::Red),
        r"{c2}" => Ok(Color::Green),
        r"{c3}" => Ok(Color::Blue),
        r"{c4}" => Ok(Color::Yellow),
        r"{c5}" => Ok(Color::Cyan),
        r"{c6}" => Ok(Color::Magenta),
        r"{c7}" => Ok(Color::Black),
        t => Err(Error::new(ProgramError::Conversion(t.to_string()))),
    }
}

fn split_text(text: &str) -> Result<Text> {
    let re = Regex::new(r"\{.*?\}")?;
    let color_tag = re.find(text).ok_or(ProgramError::ColorTag)?.as_str();
    let content = re.split(text).nth(1).ok_or(ProgramError::Text)?;

    Ok(Text{
        color_tag: to_color(color_tag)?,
        content: content.to_string()
    })
}

fn parse_file(mut file: File) -> Result<Vec<Text>> {
    let re = Regex::new(r"\{[^{]*")?;
    let mut file_buf = String::new();
    file.read_to_string(&mut file_buf)?;
    let prepended_file = String::from(r"{c0}") + &file_buf;

    let segments = re.find_iter(&prepended_file).map(|m| m.as_str().to_string());
    let mut texts = Vec::new();

    for s in segments {
        texts.push(split_text(&s)?);
    }

    Ok(texts)
}

fn display_text(text: Vec<Text>) {
    for segment in text {
        print!("{}", segment.content.as_str().color(segment.color_tag));
    }
}

fn main() -> Result<()> {
    let path = args().nth(1).ok_or(ProgramError::Path)?;
    
    loop {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        let file = File::open(PathBuf::from(path.clone()))?;

        match parse_file(file.try_clone()?)
        {
            Ok(v) => display_text(v),
            Err(e) => println!("{e}"),
        };
        println!();

        sleep(Duration::from_millis(250));
    }
}
