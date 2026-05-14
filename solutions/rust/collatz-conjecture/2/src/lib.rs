pub fn collatz(n: u64) -> Option<u64> {
    let mut count = 0;
    let mut n= n;
    loop {
        match n {
            0 => return None,
            1 => break,
            _ => {
            match n%2 {
                0 => {
                    n /= 2
                },
                1 => {
                    n *= 3;
                    n += 1
                }
                _ => return None
                }
            }
        }
        count += 1;
    }
    Some(count)
}
