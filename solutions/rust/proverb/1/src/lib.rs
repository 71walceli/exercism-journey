pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb_lines = String::new();
    if list.is_empty() {
        return proverb_lines;
    }
    
    let mut word_windows = list.iter();
    for (index, word) in word_windows.enumerate().skip(1) {
        proverb_lines.push_str(format!("For want of a {} the {word} was lost.\n", list[index-1]).as_str());
    }

    proverb_lines.push_str(format!("And all for the want of a {}.", list[0]).as_str());

    proverb_lines
    
}
