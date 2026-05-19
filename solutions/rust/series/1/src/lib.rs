pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len > digits.len() {
        return vec!();
    }
    
    digits.as_bytes().iter().enumerate().take(digits.len() -len +1)
        .map(|(i, _)| String::from(&digits[i..i+len]))
        .collect()
}
