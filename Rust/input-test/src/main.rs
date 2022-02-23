use std::io;

fn main() {
    let mut input = String::new();

    println!("Type:");

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("Success! You said: {}", input);
        }
        Err(e) => println!("Oops! Something went wrong: {}", e)
    }
}
