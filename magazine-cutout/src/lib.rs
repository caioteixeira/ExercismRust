// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::{hash_map, HashMap};

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazine_words = HashMap::new();

    for entry in magazine {
        match magazine_words.get(entry) {
            Some(count) => {
                magazine_words.insert(entry, count + 1);
            }
            None => {
                magazine_words.insert(entry, 1);
            }
        }
    }

    for entry in note {
        match magazine_words.get(entry) {
            Some(count) => {
                if (count <= &0) {
                    return false;
                }
                magazine_words.insert(entry, count - 1);
            }
            None => {
                return false;
            }
        }
    }

    true
}
