use std::collections::HashSet;
pub fn check(candidate: &str) -> bool {
    if candidate.len() == 0 {
        return true;
    } else {
        let mut set = HashSet::new();
        for c in candidate.chars() {
            if c.is_alphabetic() {
                if set.contains(&c) {
                    return false;
                } else {
                    set.insert(c);
                }
            }
        }
    }
    true
}
