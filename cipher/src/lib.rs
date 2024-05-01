#[derive(Debug,Clone,Eq,PartialEq)]
pub struct CipherError {
    pub validation: bool,
    pub expected: String,
}

impl CipherError {
    pub fn new(validation:bool, expected: String) -> CipherError {
        CipherError{
            validation,
            expected
        }
    }
}

pub fn cipher(original: &str, ciphered: &str) -> Option<Result<bool,CipherError>> {
    if original.is_empty() || ciphered.is_empty(){
        return None;
    }

    let expected_cipher = atbash_cipher(original);

    if expected_cipher == ciphered {
        Some(Ok(true))
    }else{
        Some(Err(CipherError::new(false,expected_cipher)))
    }
}


pub fn atbash_cipher(input: &str) -> String{
    input.chars().map(|c| {
        if c.is_ascii_alphabetic(){
            if c.is_ascii_lowercase(){
                ('z' as u8 - (c as u8 - 'a' as u8)) as char 
            }else{
                ('Z' as u8 - (c as u8 - 'A' as u8)) as char
            }
        }else{
            c
        }  
    }).collect()
}
