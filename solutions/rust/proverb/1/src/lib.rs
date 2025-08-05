pub fn build_proverb(list: &[&str]) -> String {
    let mut result_vec = Vec::new();

    if list.is_empty() {
        return String::new();
    }

    for i in 0..(list.len() - 1) {
        result_vec.push(format!("For want of a {} the {} was lost.", list[i], list[i + 1]));
    }
    
    result_vec.push(format!("And all for the want of a {}.", list[0]));

    result_vec.join("\n")
}
