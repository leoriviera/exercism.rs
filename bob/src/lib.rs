pub fn reply(message: &str) -> &str {
    let s = message.trim();

    let is_empty = s.is_empty();
    if is_empty {
        return "Fine. Be that way!";
    }

    let is_question = s.ends_with('?');
    let is_shouting = s.chars().any(|c| c.is_uppercase()) && s.to_uppercase() == s;

    if is_question && is_shouting {
        return "Calm down, I know what I'm doing!";
    }

    if is_question {
        return "Sure.";
    }

    if is_shouting {
        return "Whoa, chill out!";
    }

    "Whatever."
}
