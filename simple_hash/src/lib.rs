use std::collections::HashMap;

pub fn word_frequency_counter(words: Vec<&str>) -> HashMap<&str, usize> {
    let mut result = HashMap::new();
    for word in words {
        let count = result.entry(word).or_insert(0);
        *count += 1;
    }

    result
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    frequency_count.keys().count()
}