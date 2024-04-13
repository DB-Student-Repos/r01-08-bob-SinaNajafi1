pub fn reply(message: &str) -> &str {
    let message = message.trim();          
    let qst = message.ends_with("?");
    let Upper = message.contains(char::is_alphabetic) && message == message.to_uppercase();
    if message.is_empty() {
        "Fine. Be that way!"
    } 
    else if qst && Upper {          
        "Calm down, I know what I'm doing!"
    } 
    else if qst {
        "Sure."
    } 
    else if Upper {
        "Whoa, chill out!"
    } 
    else {
        "Whatever."
    }
}
