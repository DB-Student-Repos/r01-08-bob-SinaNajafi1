pub fn reply(message: &str) -> &str {
    let message = message.trim();          
    let qst = message.ends_with("?");
    let upper = message.contains(char::is_alphabetic) && message == message.to_uppercase();
    let silence = message.is_empty();
   if silence {
        "Fine. Be that way!"
    } 
    else if qst && upper {          
        "Calm down, I know what I'm doing!"
    } 
    else if qst {
        "Sure."
    } 
    else if upper {
        "Whoa, chill out!"
    } 
    else {
        "Whatever."
    }
}
