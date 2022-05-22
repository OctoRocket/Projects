use reqwest::blocking::get;

#[derive(Debug, Clone)]
struct Tree {
    link: String,
    children: Option<Vec<Tree>>,
}

fn links_on_website(website: String) -> Vec<String> {
    let mut links = Vec::new();
    let body = get(&website).unwrap().text().unwrap();
    // get all links
    for link in body.match_indices("<a href=\"") {
        // get the link
        let link = body[link.0..].split("\"").collect::<Vec<&str>>()[1];
        // add the link to the list
        links.push(link.to_string());
    }
    // replace first / with website/link
    let links = links.iter_mut().map(|link| {
        if link.starts_with('/') {
            *link = website.clone().trim().to_owned()+link;
            link.to_string()
        } else {
            link.to_string()
        }
    }).collect();
    links
}

fn init() -> Tree {
    // take user input for the url and get source of page
    let mut website = String::new();
    std::io::stdin().read_line(&mut website).unwrap();
    let links = links_on_website(website.clone());
    // create a tree
    let mut webtree = Tree {
        link: website.trim().to_string(),
        children: None,
    };
    // add all links to the tree
    let vec_of_trees: Vec<Tree> = links.iter().map(|s| Tree { link: s.to_owned(), children: None }).collect();
    webtree.children = Some(vec_of_trees);
    webtree
}

fn main() {
    let webtree = init();
    loop {
        // get all already found links
        let links = format!("{:?}", webtree);
        for i in links.match_indices("link: ") {
            println!("{}", links[i.0..]
                .split("\", children: ")
                .map(|s| s
                    .trim_start_matches("link: \"")
                ).collect::<Vec<&str>>()[0]);
        }
        // get links of webtree children
        // let mut vec_of_links = Vec::new();
        // for i in webtree.children.iter().flatten() {
        //     vec_of_links.push(links_on_website(i.link.clone()));
        // }
        break;
    }
}
