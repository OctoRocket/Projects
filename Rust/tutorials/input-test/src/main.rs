use std::io;

fn main() {
    let mut input = String::new();

    println!("Type:");
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("Success! You typed: {}", input.to_uppercase());
        },
        Err(e) => println!("Oops! Something went wrong: {}", e)
    }
}
