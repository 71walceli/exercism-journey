/// Determines whether the supplied string is a valid ISBN number

fn is_x(chr: char) -> bool {
    chr == 'X'
}
fn is_digit(chr: char) -> bool {
    chr.is_digit(10)
}
fn is_isbn_digit(chr: char) -> bool {
    is_digit(chr) || is_x(chr)
}

pub fn is_valid_isbn(isbn: &str) -> bool {
    // Preprocessing
    let isbn = isbn.replace("-", "").to_uppercase();
    
    // Validation
    let mut isbn_enum = isbn.chars().enumerate();
    if let Some((i, c)) = isbn_enum.by_ref().take(10).filter(|(i, c)| !is_digit(*c)).next() {
        if (is_x(c) && i != 9) || !is_x(c) {
            println!("Unexpected '{c}' at isbn[{i}]");
            return false;
        }
    }

    if let Some((i, c)) = isbn_enum.by_ref().next() {
        println!("Too many digits");
        return false;
    }

    // Processing
    let mut digits = isbn.chars().take(9).filter_map(|c| c.to_digit(10)).collect::<Vec<_>>();
    if digits.len() < 9 {
        println!("Too few digits");
        return false;
    }

    if let Some(tenth_digit) = isbn.chars().skip(9).next() {
        match tenth_digit {
            'x'|'X' => digits.push(10),
            '0'..='9' => digits.push(tenth_digit.to_digit(10).unwrap()),
            _ => unreachable!(),
        }
    }
    digits.reverse();

    // Output
    digits.into_iter().enumerate().map(|(i, d)| d*(i as u32+1)).sum::<u32>() % 11 == 0
}
