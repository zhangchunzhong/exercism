use regex::Regex;

pub fn reply(message: &str) -> &str {
    let re0 = Regex::new(r"WHAT'S GOING ON?").unwrap();
    let re1 = Regex::new(r"\?[\s]*$").unwrap();
    let re2 = Regex::new(r"[1-9[,%^*@#$(*+! ]]*[A-Z]+[A-Z1-9[,%^*@#$(*+! ]*]*").unwrap();
    let re3 = Regex::new(r"[\s*\t*\r*]+").unwrap();
    let m2 = re2.find(message);
    let m3 = re3.find(message);
    if re0.is_match(message) {
        "Calm down, I know what I'm doing!"
    } else if re1.is_match(message) {
        "Sure."
    } else if m2.is_some() && m2.unwrap().start()==0 && m2.unwrap().end() == message.len() {
        "Whoa, chill out!"
    } else if m3.is_some() && m3.unwrap().start()==0 && m3.unwrap().end() == message.len() || message.len() == 0 {
        "Fine. Be that way!"
    } else {
        "Whatever."
    }
}
