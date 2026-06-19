use nom::{
    IResult,
    Parser,
    bytes::complete::{
        tag,
        take_till,
        take_until1,
    },
    sequence::{
        delimited,
        terminated,
        preceded,
    },
    character::complete::digit1,
    multi::many0,
    error::{Error, ErrorKind},
    Err,
};

pub fn all_ids(input: &str) -> Vec<u32> {
    many0(first_id).parse(input).unwrap_or_default().1.into_iter().map(|s| s.parse::<u32>().unwrap()).collect()
}

pub fn first_id(input: &str) -> IResult<&str, &str> {
    for (i, _) in input.char_indices() {
        if let Ok(o) = steam_id(&input[i..]) {
            return Ok(o);
        }
    }

    IResult::Err(Err::Error(Error::new(input, ErrorKind::Fail)))
}

pub fn steam_id(input: &str) -> IResult<&str, &str> {
    string.and_then(preceded((https, domain, take_till(|c: char| c.is_ascii_digit())), digit1)).parse(input)
}

fn string(input: &str) -> IResult<&str, &str> {
    delimited(tag("\""), take_until1("\""), tag("\"")).parse(input)
}

fn https(input: &str) -> IResult<&str, &str> {
    terminated(tag("https"), tag("://")).parse(input)
}

fn domain(input: &str) -> IResult<&str, &str> {
    // take_till1(|c| c == '/').parse(input)
    tag("shared.fastly.steamstatic.com").parse(input)
}
