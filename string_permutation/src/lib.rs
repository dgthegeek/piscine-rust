
fn main() {
	let word = "codde";
	let word1 = "deeco";
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
        }else{
            if s1.chars().filter(|&e| e == c).count() != s2.chars().filter(|&e| e == c).count(){
                return false 
            }
        }
    }

    true
}

