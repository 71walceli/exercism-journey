use regex::Regex;
use std::sync::LazyLock;


static COMPOUND_WORD_REGEX: LazyLock<Regex> = LazyLock::new(|| 
    Regex::new(r"(?P<previous>[a-z]+)(?P<cur_initial>[A-Z]+)").unwrap()
);

pub fn abbreviate(phrase: &str) -> String {
    let phrase = phrase.replace("-", " ");
    
    phrase.split(' ')
        // 1. Make sure to have full words with alphabetic chars -- no synbols or punctuation
        .filter_map(|word| word.chars()
            .filter(|c| ('A'..='Z').chain('a'..='z').any(|c_| c_ == c.to_ascii_uppercase()))
            .next()
            .map(|_| word)
        )
        // 2. Normalize every word it. E.g. CPU -> Cpu
        .map(|word| {
            if word == word.to_uppercase() || word == word.to_lowercase() {
                format!("{}{}", 
                    word.chars().next().unwrap().to_ascii_uppercase(), 
                    word[1..].to_lowercase()
                )
            } else {
                String::from(word)
            }
        })
        // 3. Separate compound words
        .map(|word| String::from(
            COMPOUND_WORD_REGEX.replace_all(word.as_str(), "${previous}_${cur_initial}")
        ))
        // 4. Finally get every word's initial
        .map(|word| word.chars()
            .filter(|c| ('A'..='Z').contains(&c))
            .map(|i| i.to_ascii_uppercase())
            .collect::<String>()
        )
        .collect()
}
