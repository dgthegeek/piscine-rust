use std::collections::HashMap;

pub fn word_frequency_counter(words: Vec<&str>) -> HashMap<&str, usize> {
    let mut result_map: HashMap<&str, usize> = HashMap::new();

    for word in &words {
        let mut count = 0;
        for wor in &words {
            if wor == word{
                count+=1
            }
        }
        result_map.insert(&word, count);
    }

    result_map
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    frequency_count.len()
}

fn main() {
    let sentence = "this is a very basic sentence with only few \
                    repetitions. once again this is very basic and \
                    but it should be enough for basic tests".to_string();
    let words = sentence.split_whitespace().collect::<Vec<&str>>();

    let frequency_count = word_frequency_counter(words.clone());
    println!("{:?}", frequency_count);
    println!("{}", nb_distinct_words(&frequency_count));
}
