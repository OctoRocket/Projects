fn longest_common_prefix(mut strs: Vec<String>) -> String {
        strs.sort_unstable_by_key(|s| s.len());
        let mut solution = String::new();
        for i in 0  ..strs[0].len() {
            if strs
                .iter()
                .all(|s| s.chars().nth(i) == strs[0].chars().nth(i))
            {
                solution.push(strs[0].chars().nth(i).unwrap());
            } else {
                break;
            }
        }
        solution
}

fn main() {
    println!(
        "{}",
        longest_common_prefix(Vec::from(
            ["flower", "flow", "flight"].map(String::from)
        ))
    );
}
