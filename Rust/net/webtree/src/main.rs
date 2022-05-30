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
    // replace first / with website/link and also fix other nonsense
    let mut new_links = Vec::new();
    for link in links {
        if link.starts_with('/') {
            new_links.push(website.clone().trim().to_owned()+&link);
        } else if link.starts_with("./") {
            new_links.push(website.clone().trim().to_owned()+&link);
        } else if link.starts_with("../") {
            let mut trimmed = website.clone().chars().next_back().unwrap().to_string();
            while trimmed.chars().last().unwrap() != '/' {
                trimmed = trimmed.chars().next_back().unwrap().to_string();
            }
            new_links.push(trimmed.trim().to_owned()+&link);
        } else {
            new_links.push(link.to_string());
        }
    }
    let mut newer_links = Vec::new();
    for link in new_links.iter_mut() {
        if link.starts_with("https://") || link.starts_with("http://") {
            newer_links.push(link.to_string());
        }
    }
    newer_links
}

fn get_found_links(tree: &Tree) -> Vec<String> {
    let links = format!("{:?}", tree);
    let mut found_links = Vec::new();
    for i in links.match_indices("link: ") {
        found_links.push(links[i.0..]
            .split("\", children: ")
            .map(|s| s
                .trim_start_matches("link: \"")
            ).collect::<Vec<&str>>()[0].to_string());
    }
    found_links
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
    let mut webtree = init();
    loop {
        // get all already found links
        let found_links = get_found_links(&webtree);
        // get links of webtree children
        let mut new_children = Vec::new();
        for i in webtree.children.iter().flatten() {
            let mut child_children = Vec::new();
            links_on_website(i.link.clone()).iter().for_each(|s| {
                if !found_links.contains(&s) {
                    child_children.push(Tree { link: s.to_owned(), children: None });
                }
            });
            new_children.push(Tree { link: i.link.clone(), children: Some(child_children) });
        }
        // add new children to webtree
        webtree.children = Some(new_children);
        break;
    }
    for i in get_found_links(&webtree) {
        println!("{}", i);
    }
}
