mod num2string;

fn _plural(bottles: u32) -> &'static str {
    match bottles {
        1 => "",
        _ => "s",
    }
}

fn _recite_chore(bottle_count: u32) -> String {
    let n_bottles = num2string::encode(bottle_count.into()).chars().enumerate()
        .map(|(i, c)| {
            if i == 0 {
                c.to_uppercase().next().unwrap()
            } else {
                c
            }
        }).collect::<String>()
    ;
    let next_n_bottles = num2string::encode((bottle_count -1).into());
    
    let plural1 = _plural(bottle_count);
    let plural2 = _plural(bottle_count - 1);
    
    let _1st_sentence = format!("{n_bottles} green bottle{plural1} hanging on the wall,");
    let _4th_sentence = format!(
        "There'll be {next_n_bottles} green bottle{plural2} hanging on the wall."
    );
    
    let mut result = String::new();
    result.push_str(&_1st_sentence);
    result.push('\n');
    result.push_str(&_1st_sentence);
    result.push('\n');
    result.push_str("And if one green bottle should accidentally fall,");
    result.push('\n');
    result.push_str(&_4th_sentence);
    
    result
}

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let start = start_bottles - take_down +1;
    let finish = start_bottles;
    
    (start..=finish).rev().map(|bottles| {
        let chore = _recite_chore(bottles);
        format!("{chore}\n\n")
    }).collect::<String>()
}
