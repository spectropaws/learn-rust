use std::io;
use std::collections::HashMap;

fn main() {
    println!("Enter a sentence: ");
    
    let mut sentence = String::new();
    match io::stdin().read_line(&mut sentence) {
        Ok(_) => {},
        Err(_) => {
            println!("Unable to read input!");
            return;
        }
    }

    let chars = sentence.trim().chars().count();
    let mut word_count: usize = 0;

    let mut map: HashMap<String, usize> = HashMap::new();
    for word in sentence.split_whitespace() {
        word_count += 1;
        *map.entry(word.to_lowercase()).or_insert(0) += 1;
    }

    let maxkey = match map.iter().max_by_key(|entry| entry.1) {
        Some((key, _)) => key,
        None => {
            println!("No words found");
            return;
        }
    };

    println!("Number of characters: {}", chars);
    println!("Number of words: {}", word_count);
    println!("Most frequent: {}", maxkey);
}
