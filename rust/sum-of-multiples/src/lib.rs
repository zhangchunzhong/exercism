pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut sum:u32 = 0;
    let mut vec = Vec::new();
    for n in 1..limit {
        for f in factors {
            if *f != 0 && n % f == 0 {
                vec.push(n)
            }
        }
    }
    vec.dedup();
    for i in vec.iter() {
        sum += i;
    }
    sum
}
