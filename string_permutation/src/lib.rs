
fn main() {
	let word = "cde";
	let word1 = "edbca";
	println!(
		"Is `{}` a permutation of `{}`? = {}",
		word,
		word1,
		is_permutation(word, word1)
	);
}


pub fn is_permutation(s1: &str, s2: &str) -> bool {
   if s1.len() != s2.len(){
    return false
   }
    for c in s1.chars(){
        if !s2.contains(c){
            return false
        }
    }

    true
}