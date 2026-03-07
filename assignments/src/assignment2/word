fn most_frequent_word(text: &str) -> (String, usize) {
    let words: Vec<&str> = text.split_whitespace().collect();

    let mut unique_words: Vec<&str> = Vec::new();
    let mut counts: Vec<usize> = Vec::new();

    for &word in &words {
        let mut found_index: Option<usize> = None;

        for i in 0..unique_words.len() {
            if unique_words[i] == word {
                found_index = Some(i);
                break;
            }
        }

        match found_index {
            Some(i) => counts[i] += 1,
            None => {
                unique_words.push(word);
                counts.push(1);
            }
        }
    }

    let mut max_word = "";
    let mut max_count = 0;

    for i in 0..unique_words.len() {
        if counts[i] > max_count {
            max_count = counts[i];
            max_word = unique_words[i];
        }
    }

    (max_word.to_string(), max_count)
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}