pub fn reply(message: &str) -> &str {
    if message.trim().is_empty() {
        "Fine. Be that way!"
    } else if message.trim().ends_with('?') && message.chars().filter(|m| m.is_alphabetic()).count() > 0 && message.chars().filter(|m| m.is_alphabetic()).all(|m| m.is_alphabetic() && matches!(m, 'A'..='Z')) {
        "Calm down, I know what I'm doing!"
    } else if message.chars().filter(|m| m.is_alphabetic()).count() > 0 && message.chars().filter(|m| m.is_alphabetic()).all(|m| matches!(m, 'A'..='Z')) {
        "Whoa, chill out!"
    } else if message.trim().ends_with('?') {
        "Sure."
    } else {
        "Whatever."
    }
}
