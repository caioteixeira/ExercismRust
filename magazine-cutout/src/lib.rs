// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::{hash_map, HashMap};

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazine_words = HashMap::new();

    for magazine_word in magazine {
        let entry = magazine_words.entry(magazine_word).or_insert(0);
        *entry += 1;
    }

    for note_word in note {
        let entry = magazine_words.entry(note_word).or_insert(0);

        if *entry == 0 {
            return false;
        }

        *entry -= 1;
    }

    true
}
