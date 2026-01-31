pub fn raindrops(n: u32) -> String {
    let mut result = String::new();
    
    if n.is_multiple_of(3) {
        result.push_str("Pling");
    }
    if n.is_multiple_of(5) {
        result.push_str("Plang");
    }
    if n.is_multiple_of(7) {
        result.push_str("Plong");
    } 
    
    if result.is_empty() {
        format!("{n}")
    } else {
        result
    }
}
