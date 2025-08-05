pub fn series(digits: &str, len: usize) -> Vec<String> {
    if digits.len() < len {
        return vec![];
    }

    if digits.len() == len {
        return vec![digits.to_string()];
    }

    let mut result = Vec::new();
    let digits_vec: Vec<&str> = digits.split("").filter(|d| d != &"").collect();

    for i in 0..digits_vec.len() {
        if i + len - 1 < digits_vec.len() {
            result.push(digits_vec[i..(i + len)].join(""))
        }
    }

    result
}
