fn longest_common_prefix(strs: &[&str]) -> String {
    let mut i = 0;
    while !strs.iter().any(|f| f.chars().nth(i) != strs[0].chars().nth(i)) {
        i += 1;
    }
    String::from(&strs[0][0..i])
}

fn main() {
    for _i in 1..=1000000 {
        longest_common_prefix(&["flower", "flowest", "flowing"]);
    }
}
