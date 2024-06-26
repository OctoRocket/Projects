use anyhow::{
    Result,
    anyhow,
};
use std::{
    io::stdin,
    fs::read_to_string,
    collections::HashMap,
};

type HashPair = (HashMap<String, String>, HashMap<String, String>);

fn take_input() -> Result<String> {
    let mut buf = String::new();
    stdin().read_line(&mut buf)?;
    Ok(buf)
}

fn parse(file: String) -> Result<Vec<Vec<String>>> {
    let file = read_to_string(file)?; // Read file
    let ftl = file
        .split('\n') // Split by new line
        .filter(|line| line != &"") // Remove empty lines
        .map(|line| line.split(" = ") // Split into the fore and the aft section
            .map(|word| word.to_string()) // Convert to string
            .collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();
    Ok(ftl)
}

fn file_to_hashes(file: String) -> Result<HashPair> {
    let ftl = parse(file)?;
    let mut replaced_words: HashMap<String, String> = HashMap::new();
    let mut replacement_words: HashMap<String, String> = HashMap::new();

    for line in ftl {
        let pair = (&line[0], &line[1]);
        if line[0].contains("replaced") { // If the word is a replaced word
            replaced_words.insert(pair.1.clone(), pair.0.clone());
        } else if line[0].contains("replacement") { // If the word is a replacement word
            replacement_words.insert(pair.0.replace("replacement", "replaced"), pair.1.clone());
        } else {
            return Err(anyhow!("Invalid FTL file"));
        }
    }
    Ok((replaced_words, replacement_words))
}

fn replace_in_sentence(mut sentence: String, hash_pair: &HashPair) -> String {
    let (replaced_words, replacement_words) = hash_pair;

    // For each word, check if it's contained in the hash and replace it if so.
    for word in sentence.clone().split(' ') {
        if replaced_words.contains_key(word) {
            sentence = sentence.replace(word, replacement_words[&replaced_words[word]].as_str())
        }
    }

    sentence
}

fn main() -> Result<()> {
    // Get file input
    println!("Path to FTL file (with comments removed)");
    let file_input = take_input()?.trim().to_string();

    // Get the hashes
    let hash_pair = file_to_hashes(file_input)?;

    // Get user input
    println!("Input a sentence");
    loop {
        let input = take_input()?.to_lowercase().trim().to_string();

        println!("{}", replace_in_sentence(input, &hash_pair));
    }
}
