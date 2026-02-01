pub fn reply(message: &str) -> &str {
    let message = message.trim();
    if message.is_empty() {
        return "Fine. Be that way!";
    }
    let message_has_question_mark = *message.as_bytes().last().unwrap() == b'?';

    let message_is_yelling = message.as_bytes().iter().any(|chr| chr.is_ascii_lowercase() || chr.is_ascii_uppercase())
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
