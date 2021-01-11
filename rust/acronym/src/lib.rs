pub fn abbreviate(phrase: &str) -> String {
    let mut sp = ' ';
    let mut preup:bool = false;
    let mut ret = String::new();
    for c in phrase.chars() {
        if (sp == '-' || sp == ' ' || (c.is_ascii_uppercase() && !preup)) && c.is_ascii_alphabetic() {
            ret.push(c.to_ascii_uppercase());
        }
        sp = c;
        preup = c.is_ascii_uppercase();
    }
    ret
}
