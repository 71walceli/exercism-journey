pub fn reply(message: &str) -> &str {
    let message = message.trim();
    if message.is_empty() {
        return "Fine. Be that way!";
    }
    let message_has_question_mark = *message.as_bytes().last().unwrap() == '?' as u8;

    let message_is_yelling = message.chars().any(|chr| ('a'..='z').contains(&chr) || ('A'..='Z').contains(&chr))
        && message.to_uppercase() == message
    ;
    if message_is_yelling {
        return if message_has_question_mark {
            "Calm down, I know what I'm doing!"
        } else {
            "Whoa, chill out!"
        };
    }

    if message_has_question_mark {
        "Sure."
    } else {
        "Whatever."
    }
}
