

pub fn square(s: u32) -> u64 {
    let mut grains_sum:u64 = 1;
    let mut grains:u64 = 1;
    if s <= 0 || s > 64 {
        panic!("Square must be between 1 and 64.");
    }
    for _ in 0..s-1 {
        grains_sum += grains as u64;
        //println!("square {}  grains = {}  total= {}", i, grains, grains_sum);
        grains = grains * 2;
    }
    grains_sum
}

pub fn total() -> u64 {
    let mut sum:u64 = 0;
    for n in 1..=64 {
        sum += square(n);
    }
    sum
}
