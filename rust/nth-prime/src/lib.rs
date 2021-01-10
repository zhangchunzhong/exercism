
pub fn is_prime(n: u32) -> bool {
    let limit = (n as f64).sqrt() as u32;

    for i in 2..=limit {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn nth(n: u32) -> u32 {
    if n == 0 {
        2
    } else {
        let mut count = n;
        let mut max : u32 = 3;
        while count > 0 {
            if is_prime(max) {
                count -= 1;
                if count > 0 {
                    max += 1;
                }
            } else {
                max += 1;
            }
        }
        max
    }
}
