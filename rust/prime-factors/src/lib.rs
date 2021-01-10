pub fn factors(n: u64) -> Vec<u64> {
    let mut vec = Vec::new();
    let mut m = n;
    while m > 1 {
        for div in 2..=m {
            if m % div == 0 {
                vec.push(div);
                m = m / div;
                break;
            }
        }
    }
    vec
}
