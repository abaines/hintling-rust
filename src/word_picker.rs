use std::cell::Cell;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// Embed the word list at compile time
const WORDS: &str = include_str!("../words.txt");

// Simple counter to vary the seed
thread_local! {
    static COUNTER: Cell<usize> = Cell::new(0);
}

/// Get a random word from the embedded word list using a simple counter-based hash
pub fn get_random_word() -> &'static str {
    let words: Vec<&str> = WORDS.lines().filter(|line| !line.trim().is_empty()).collect();
    
    if words.is_empty() {
        return "word";
    }
    
    // Increment counter and hash it for variety
    let count = COUNTER.with(|c| {
        let val = c.get();
        c.set(val.wrapping_add(1));
        val
    });
    
    let mut hasher = DefaultHasher::new();
    count.hash(&mut hasher);
    let hash = hasher.finish() as usize;
    let index = hash % words.len();
    
    words[index]
}
