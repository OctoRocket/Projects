use reqwest::{blocking::get};

fn links_on_website(website: String) -> Vec<String> {
    let mut links = Vec::new();
    let body = get(website).unwrap().text().unwrap();
    // get all links
    for link in body.match_indices("<a href=\"") {
        // get the link
        let link = body[link.0..].split("\"").collect::<Vec<&str>>()[1];
        // add the link to the list
        links.push(link.to_string());
    }
    // replace first / with https://
    let mut final_links = Vec::new();
    for link in 0..links.len() {
        if links[link].chars().nth(0).unwrap() == '/' {
            final_links.push("https://".to_owned() + &links[link][1..links[link].len()]);
        } else if links[link].chars().nth(0).unwrap() != '#' {
            final_links.push(links[link].to_owned());
        }
    }
    final_links
}

fn main() {
    // take user input for the url and get source of page
    let mut website = String::new();
    std::io::stdin().read_line(&mut website).unwrap();
    let links = links_on_website(website);
    for i in links {
        println!("{}", i);
    }
}
