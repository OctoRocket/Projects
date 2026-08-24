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

// ID extraction

pub fn all_ids(input: &str) -> Vec<String> {
    many0(first_id).parse(input).unwrap_or_default().1.into_iter().map(String::from).collect()
}

fn first_id(input: &str) -> IResult<&str, &str> {
    for (i, _) in input.char_indices() {
        if let Ok(o) = steam_id(&input[i..]) {
            return Ok(o);
        }
    }

    IResult::Err(Err::Error(Error::new(input, ErrorKind::Fail)))
}

fn steam_id(input: &str) -> IResult<&str, &str> {
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

// Steam title extraction

pub fn title(input: &str) -> &str {
    (tag::<&str, &str, nom::error::Error<_>>("<title>"), take_until1("</title>"), tag("</title>")).parse(input).unwrap().1.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn title_test() {
        let data = r#"<head>
            <title>Asta!</title>
            <link rel="stylesheet" href="/style/update-log.css">
            <link rel="stylesheet" href="/style/style.css">
            <link rel="icon" type="image/x-icon" href="/favicon.png">
            <script type="module" src="./scripts/site-updates.js"></script> <!-- Automatically updates the site update log -->
            <meta charset="utf8">
            <meta http-equiv="Content-type" content="text/html; charset=utf-8">
            <meta name="viewport" content="width=device-width, initial-scale=1">
            <meta name="author" content="Asta/OctoRocket">
            <meta name="description" content="My home page: a satellite drifting through space">
        </head>"#;

        dbg!(title(data));
    }
}
