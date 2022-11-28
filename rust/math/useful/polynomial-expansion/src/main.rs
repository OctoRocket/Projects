use std::fmt;

#[derive(Debug, Clone)]
struct CheckError;

impl fmt::Display for CheckError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Imput failed the input check")
    }
}

fn input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn check_input(input: String) -> Result<Vec<String>, CheckError> {
    /*
    make sure that the input is vailid
    It should look like "(x^2+x+2)(x^6+x)(8)(x-2)
    */
    // first check if it has the format ()()()()
    let input_check: Vec<&str> = input.split(")(").collect();
    let clean_input: Vec<String>;
    if input_check[0].starts_with("(") && input_check[input_check.len() - 1].ends_with(")") {
        clean_input = input_check
                                 .iter()
                                 .map(|f| f.replace("(", "")
                                 .replace(")", ""))
                                 .collect();
    } else {
        println!("Incorrect format");
        println!("{:?}", input_check);
        return Err(CheckError);
    }
    // next check if each () has something
    for i in &clean_input {
        if i.trim() == "" {
            return Err(CheckError);
        }
    }
    return Ok(clean_input);
}

fn main() {
    let inp = input();
    let clean_input = check_input(inp);
    match clean_input {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("{}", e)
    };
}
