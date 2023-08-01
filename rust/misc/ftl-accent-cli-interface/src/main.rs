use std::{
    io::stdin,
    fs::read_to_string,
    collections::HashMap,
};

#[derive(Debug)]
struct FtlToJsonError {
    // This code isn't actually dead, it is used when the program errors.
    #[allow(dead_code)]
    error: String,
}

impl From<std::io::Error> for FtlToJsonError {
    fn from(e: std::io::Error) -> Self {
        FtlToJsonError{ error: e.to_string() }
    }
}

type FtlToJsonResult = Result<(HashMap<String, String>, HashMap<String, String>), FtlToJsonError>;

fn take_input() -> Result<String, FtlToJsonError> {
    let mut buf = String::new();
    stdin().read_line(&mut buf)?;
    Ok(buf)
}

fn file_to_hashes(file: String) -> FtlToJsonResult {
    let binding = read_to_string(file)?;
    let mut to_hash: HashMap<String, String> = HashMap::new();
    let mut from_hash: HashMap<String, String> = HashMap::new();
    let ftl = binding
        .split('\n')
        .filter(|line| line != &"")
        .map(|line| line.split(" = ").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    for line in ftl {
        let stringified_line = (line[0].to_string(), line[1].to_string());
        if line[0].contains("replaced") {
            to_hash.insert(stringified_line.1, stringified_line.0);
        } else if line[0].contains("replacement") {
            from_hash.insert(stringified_line.0.replace("replacement", "replaced"), stringified_line.1);
        } else {
            return Err(FtlToJsonError{ error: "File formatted incorrectly.".to_string() });
        }
    }
    Ok((to_hash, from_hash))
}

fn main() -> Result<(), FtlToJsonError> {
    println!("Path to FTL file (with comments removed)");
    let file_input = match take_input()?.trim().to_string() {
        s if s == "" => "archaic.ftl".to_string(),
        s => s,
    };
    let (to_hash, from_hash) = file_to_hashes(file_input)?;
    println!("Input a sentance");
    loop {
        let mut input = take_input()?.to_lowercase().trim().to_string();
        for word in input.clone().split(' ') {
            if to_hash.contains_key(word) {
                input = input.replace(word, from_hash[&to_hash[word]].as_str());
            }
        }
        println!("{input}");
    }
}
