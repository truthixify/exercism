use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut set = HashSet::new();

    for anagram in possible_anagrams {
        if word.to_lowercase() == *anagram.to_lowercase() {
            continue;
        }
        
        let mut word_chars: Vec<_> = word.to_lowercase().chars().collect();
        let mut anagram_chars: Vec<_> = anagram.to_lowercase().chars().collect();
        
        word_chars.sort();
        anagram_chars.sort();

        if word_chars == anagram_chars {
            set.insert(*anagram);
        }
    }

    set
}
