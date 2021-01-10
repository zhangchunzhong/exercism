
pub fn brackets_are_balanced(string: &str) -> bool {
    let mut vec = Vec::new();
    for  ch in string.chars() {
        if ch == '{' || ch == '[' || ch == '(' {
            vec.push(ch);
        }
        if ch == '}' || ch == ']' || ch == ')' {
            let ret = match vec.pop() {
                None => false,
                Some(p) => {
                    match p {
                        '{' => ch == '}',
                        '[' => ch == ']',
                        '(' => ch == ')',
                        _ => false,
                    }
                },
            };
            if ret == false {return false}
        }
    }
    if vec.len() == 0 {
        true
    } else {
        false
    }
}
