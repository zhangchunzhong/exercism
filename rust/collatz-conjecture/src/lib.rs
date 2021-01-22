pub fn collatz(n: u64) -> Option<u64> {
   let mut num = n;
   let mut count:u64 = 0;
    while num >= 1 {
        if num == 1 {
            return Some(count)
        }
        if num % 2 == 0 {
            num = num / 2;
            count += 1;
        } else {
            num = 3 * num + 1;
            count += 1;
        }
   }
   None
}
