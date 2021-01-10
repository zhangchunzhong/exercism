pub fn build_proverb(list: &[&str]) -> String {
    let mut s = String::new();
    let sz = list.len();
    for i in 0..sz {
        if i == sz - 1 {
            s.push_str(&format!("And all for the want of a {}.", list[0]));
        } else {
            s.push_str(&format!("For want of a {} the {} was lost.\n", list[i],list[i+1]));
        }
    }
    s
}
