
pub fn brackets_are_balanced(string: &str) -> bool {
    let openings = "({[";
    let closings = ")}]";

    let mut stack = vec![];
    for chr in string.as_bytes().iter() {
        if let Some(opening_kind) = openings.as_bytes().iter().position(|_chr| _chr == chr) {
            stack.push(opening_kind);
        }
        if let Some(closing_kind) = closings.as_bytes().iter().position(|_chr| _chr == chr) {
            if stack.last().is_some_and(|x| *x == closing_kind) {
                stack.pop();
            } else {
                return false;
            }
        }
    }

    stack.is_empty()
}
