pub fn is_armstrong_number(num: u32) -> bool {
    let mut n = num;
    let mut sum = 0;
    let mut len_ = 0;
    while n != 0 {
        len_ += 1;
        n /= 10;
    }
    let mut n = num;
    while n != 0 {
        let temp = n % 10;
        n /= 10;
        sum += temp.pow(len_);
    }
    sum == num
}
