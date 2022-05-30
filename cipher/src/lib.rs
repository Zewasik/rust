use std::str;

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
    if ciphered.len() == 0 || original.len() == 0 {
        return None;
    }

    let mut temp_str = String::new();

    for ch in original.chars() {
        if ch >= 'A' && ch <= 'Z' {
            temp_str += str::from_utf8(&[b'Z' + b'A' - ch as u8]).unwrap()
        } else if ch >= 'a' && ch <= 'z' {
            temp_str += str::from_utf8(&[b'a' + b'z' - ch as u8]).unwrap()
        } else {
            temp_str += &ch.to_string()
        }
    }

    if temp_str == ciphered {
        Some(Ok(true))
    } else {
        Some(Err(CipherError::new(false, temp_str.to_string())))
    }
}
