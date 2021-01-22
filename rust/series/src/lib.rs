pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut rvec = Vec::new();
    if len == 0 {
        for _i in 0..digits.len() + 1 {
            rvec.push("".to_string())
        }
    } else if digits.len() >= len {
        let l = digits.len() - len + 1;
        for i in 0..l {
            rvec.push(digits[i..(i+len)].to_string());
        }
    }
    rvec
}
