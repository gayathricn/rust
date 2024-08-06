pub fn reply(message: &str) -> &str {
    //todo!("have Bob reply to the incoming message: {message}")
    let trimmed_message =message.trim();

    if trimmed_message.is_empty(){
        "Fine. Be that way!"
    }else if trimmed_message.chars().any(|c| c.is_alphabetic()) && 
    trimmed_message.chars().all(|c| !c.is_lowercase()) {
        if trimmed_message.ends_with('?') {
            "Calm down, I know what I'm doing!"
        } else {
            "Whoa, chill out!"
        }
    }else if trimmed_message.ends_with('?') {
        "Sure."
    }else {
        "Whatever."
    }
}