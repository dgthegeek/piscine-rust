pub fn is_ascii(v: &str) -> bool {
    for c in v.chars() {
        if c as u32 > 127 {
            return false;
        }
    }
    true
}

pub fn find(v: &str, pat: char) -> usize {
    v.find(pat).unwrap_or(0)
}

pub fn is_empty(v: &str) -> bool {
    v.is_empty()
}

pub fn contains(v: &str, pat: &str) -> bool {
    v.contains(pat)
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    v.split_at(index)
}
