/// This file contains all the functions used in the program.
/// This file is used to keep the main file clean.

/// This trait is used to convert a set of characters into a vector of Sets.
pub trait ConvertToSets {
    fn convert_to_sets(&self) -> Vec<Sets>;
}

/// This function converts the sets of characters into a vector of Sets.
impl ConvertToSets for String {
    fn convert_to_sets(&self) -> Vec<Sets> {
        let mut sets = Vec::new();
        for c in self.chars() {
            match c {
                'l' => sets.push(Sets::Lowercase),
                'u' => sets.push(Sets::Uppercase),
                'd' => sets.push(Sets::Digits),
                's' => sets.push(Sets::Symbols),
                _ => (),
            }
        }
        sets
    }
}

/// A set of characters.
/// This has 4 sets of characters, lowercase, uppercase, digits, and symbols.
#[derive(Debug)]
pub enum Sets {
    Lowercase,
    Uppercase,
    Digits,
    Symbols,
}