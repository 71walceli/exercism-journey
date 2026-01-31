pub fn build_proverb(list: &[&str]) -> String {
    let mut lines = String::new();
    if list.is_empty() {
        return lines;
    }
    
    let word_windows = list.iter();
    for (index, word) in word_windows.enumerate().skip(1) {
        lines.push_str(format!("For want of a {} the {word} was lost.\n", list[index-1]).as_str());
    }

    lines.push_str(format!("And all for the want of a {}.", list[0]).as_str());

    lines
}
