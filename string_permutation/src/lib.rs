

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