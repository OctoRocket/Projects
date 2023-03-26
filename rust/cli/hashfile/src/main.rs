use std::{
    env::args,
    fs::File,
    io::{BufReader, Read},
};
use sha2::{
    Digest,
    Sha256
};

// Trait for hashing files
trait HashFile {
    fn hash(&self) -> String;
}

// Implement the trait for the File struct
impl HashFile for File {
    fn hash(&self) -> String {
        // Variables
        let mut hasher = Sha256::new();
        let mut buffer_reader = BufReader::new(self);
        let mut buffer = [0; 1024];
        
        // Hash the file
        while let Ok(bytes_read) = buffer_reader.read(&mut buffer) {
            if bytes_read == 0 {
                break;
            }
            hasher.update(&buffer[..bytes_read]);
        }

        // Return the hash
        format!("{:x}", hasher.finalize())
    }
}

// This program takes in a file as input and outputs the file after it has been encrypted
fn main() {
    // Variables
    let file = match File::open(&args().collect::<Vec<String>>()[1]) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error: File not found ({})", e);
            return;
        },
    };

    // Hash the file
    println!("{}", file.hash());
}
