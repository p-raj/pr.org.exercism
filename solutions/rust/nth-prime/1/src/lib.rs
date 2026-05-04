pub fn nth(n: u32) -> u32 {
    let mut count = 0;
    let mut check = 2;

    while count <= n {
        if is_prime(check) {
            count += 1;
        }
        check += 1;
    }
    check - 1
}

fn is_prime(n: u32) -> bool {
    for i in 2..=n/2 {
        if n % i == 0 {
            return false;
        }
    }
    true
}