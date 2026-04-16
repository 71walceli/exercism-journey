use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut mapping = BTreeMap::new();
    
    for (score, letters) in h.iter() {
        for letter in letters {
            let letter = letter.to_ascii_lowercase();
            mapping.entry(letter).or_insert(*score);
        }
    }
    
    mapping
}
