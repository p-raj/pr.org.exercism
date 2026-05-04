pub fn factors(n: u64) -> Vec<u64> {
    //todo!("This should calculate the prime factors of {n}")
    let mut n = n;
    let mut fset = Vec::<u64>::new();
    let mut i = 2;
    while i <= n {
        if n % i == 0 {
            n /= i;
            fset.push(i);
        } else {
            i = get_next_prime(i);
        }
    }
    fset
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    let end = (n as f64).sqrt() as u64;

    !(3..=end).step_by(2).any(|i| n % i == 0)
}

fn get_next_prime(n: u64) -> u64 {
    (n + 1..).find(|&x| is_prime(x)).unwrap()
}
