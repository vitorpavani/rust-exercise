pub fn reply(message: &str) -> &str {
    let message = message.trim();
    if message.is_empty() {
        return "Fine. Be that way!";
    }
    let is_question = message.ends_with('?');
    let is_yelling = message.to_uppercase() == message && message.to_lowercase() != message;

    match (is_question, is_yelling) {
        (true, false) => "Sure.",
        (false, true) => "Whoa, chill out!",
        (true, true) => "Calm down, I know what I'm doing!",
        (false, false) => "Whatever.",
    }
}
