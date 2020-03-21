fn message_is_upper(message: &str) -> bool {
    message.chars().any(|c| c.is_alphabetic())
        && message
            .chars()
            .filter(|c| c.is_alphabetic())
            .all(|c| c.is_uppercase())
}

fn message_is_empty(message: &str) -> bool {
    message.len() == 0 || (message.ends_with("?") && message.len() == 1)
}

fn message_is_question(message: &str) -> bool {
    message.ends_with("?")
}

pub fn reply(message: &str) -> &str {
    match message.trim() {
        x if message_is_question(x) && message_is_upper(x) => "Calm down, I know what I'm doing!",
        x if message_is_upper(x) => "Whoa, chill out!",
        x if message_is_empty(x) => "Fine. Be that way!",
        x if message_is_question(x) => "Sure.",
        _ => "Whatever.",
    }
}
