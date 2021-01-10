pub fn pow_num(num:u32)-> u32 {
    if num == 0 {
        0
    } else {
        let mut n = num;
        let mut c:u32 =1;
        while n > 0 {
            n = n / 10;
            if n > 0 { c+= 1; }
        }
        c
    }
}
pub fn is_armstrong_number(num: u32) -> bool {
    let mut n = num;
    let mut sum:u32 = 0;
    let p = pow_num(num);
    while n > 0 {
        let s = n % 10;
        if p > 0 {
            sum += s.pow(p);
        }
        n = n / 10;
    }
    num == sum
}
