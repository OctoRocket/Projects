use super::types::Line;
use anyhow::Result;
use colored::Styles;
use std::{
    io::{
        self,
        BufRead,
    },
    path::Path,
    fs::File,
};

pub fn read_file<FilePath>(file: FilePath) -> Result<Vec::<String>> where FilePath: AsRef<Path> {
    let lines = io::BufReader::new(File::open(file)?)
        .lines()
        .map(Result::unwrap)
        .collect();
    Ok(lines)
}

pub fn parse(lines: Vec::<String>) -> Vec::<Line> {
    let mut result = Vec::<Line>::new();
    for line in lines {
        let mut resulting_line = Line::default();
        let split_line = line.trim().split_once('|');
        if let Some(split_line) = split_line {
            resulting_line.text = split_line
                .1
                .to_string();
            let params = split_line
                .0
                .split(';')
                .collect::<Vec<&str>>();
            if !params.is_empty() {
                resulting_line.fg_color = Some(params[0].to_string());
            }
            if params.len() >= 2 {
                resulting_line.bg_color = Some(params[1].to_string());
            }
            if params.len() == 3 {
                resulting_line.style = Some(params[2].split(',').map(|s| match s{
                    "bold" => Styles::Bold,
                    "underline" => Styles::Underline,
                    "italic" => Styles::Italic,
                    "dimmed" => Styles::Dimmed,
                    "reversed" => Styles::Reversed,
                    "blink" => Styles::Blink,
                    "hidden" => Styles::Hidden,
                    "strikethrough" => Styles::Strikethrough,
                    _ => panic!("Invalid style, {}", s)
                }).collect());
            }
        } else {
            resulting_line.text = line;
        }
        result.push(resulting_line);
    }
    result
}

pub fn str_to_rbg(string: &str) -> Result<(u8, u8, u8)> {
    let rgb = string.trim_matches(|p| p == '(' || p == ')');
    let rgb: Vec<&str> = rgb.split(',').collect();
    if rgb.len() == 3 {
        let r = rgb[0].parse::<u8>()?;
        let g = rgb[1].parse::<u8>()?;
        let b = rgb[2].parse::<u8>()?;
        Ok((r, g, b))
    } else {
        Err(anyhow::anyhow!("Invalid RGB, {}", string))
    }
}
