use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams: HashSet<&'a str> = HashSet::new();
    let mut word_chars: Vec<char> = word.to_lowercase().chars().collect();
    word_chars.sort_unstable();

    for candidate in possible_anagrams {
        if candidate.to_lowercase() == word.to_lowercase() {
            continue;
        }

        let mut anagram_vec: Vec<char> = candidate.to_lowercase().chars().collect();
        anagram_vec.sort_unstable();
        if word_chars == anagram_vec {
            anagrams.insert(candidate);
        }
    }

    anagrams
}
