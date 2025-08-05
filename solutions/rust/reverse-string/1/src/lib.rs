pub fn reverse(input: &str) -> String {
    let mut reversed_string = String::from("");
    for c in input.chars().rev() {
        reversed_string = format!("{}{}", reversed_string, c);
    }
    reversed_string
}
