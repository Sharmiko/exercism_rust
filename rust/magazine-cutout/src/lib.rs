// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazine_words = HashMap::new();
    let mut note_words = HashMap::new();

    for word in magazine {
        let counter = magazine_words.entry(word).or_insert(0);
        *counter += 1;
    }

    for word in note {
        let counter = note_words.entry(word).or_insert(0);
        *counter += 1;
    }

    for (key, value) in &note_words {
        let magazine_word_count = magazine_words.get(key);
        if magazine_word_count < Some(value) {
            return false;
        }
    }

    return true;
}
