#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CipherError {
    pub validation: bool,
    pub expected: String,
}

impl CipherError {
    pub fn new(validation: bool, expected: String) -> CipherError {
        CipherError {
            validation,
            expected,
        }
    }
}

pub fn cipher(original: &str, ciphered: &str) -> Option<Result<bool, CipherError>> {
    let mut original_ciphered = String::new();
    let original_bytes = original.as_bytes();
    
    if original.is_empty() || ciphered.is_empty() {
        return None;
    }
    
    for c in original_bytes {
        if c >= &97 && c <= &122 {
            original_ciphered.push((219 - c) as char);
        } else if c >= &65 && c <= &90 {
            original_ciphered.push((155 - c) as char);
        } else {
            original_ciphered.push(*c as char);
        }
    }

    if original_ciphered == ciphered {
        Some(Ok(true))
    } else {
        Some(Err(CipherError::new(false, original_ciphered)))
    }
}
