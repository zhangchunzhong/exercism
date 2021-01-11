pub fn is_prime(n: u64) -> bool {
    let limit = (n as f64).sqrt() as u64;

    for i in 2..=limit {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut ret = Vec::new();
    for n in 2..=upper_bound {
        if is_prime(n) {
            ret.push(n);
        }
    }
    ret
}
