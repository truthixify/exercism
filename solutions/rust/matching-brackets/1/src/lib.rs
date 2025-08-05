pub fn brackets_are_balanced(string: &str) -> bool {
    let mut waiting_for_array: Vec<char> = Vec::new();
    
    for c in string.chars() {
        match c {
            '[' => waiting_for_array.push(']'),
            '{' => waiting_for_array.push('}'),
            '(' => waiting_for_array.push(')'),
            ch if matches!(ch, ']' | '}' | ')') && Some(ch) != waiting_for_array.pop() => return false,
            _ => continue,
        }
    }

    waiting_for_array.is_empty()
}
