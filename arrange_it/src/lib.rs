

pub fn arrange_phrase(phrase: &str) -> String {
    let mut list_index:Vec<(&str, u8)> = Vec::new();
    for word in phrase.split_whitespace() {
        for c in  word.chars() {
            if c.is_numeric(){
                list_index.push((word, c as u8))
            }
        }
    }
    let mut sorted_words = list_index.clone();
    sorted_words.sort_by_key(|&(_, index)| index);
    let mut new_string: String = sorted_words.iter().map(|&(word, _)| word).collect::<Vec<&str>>().join(" ");
    
    new_string.retain(|c| !c.is_numeric());
    new_string
}

