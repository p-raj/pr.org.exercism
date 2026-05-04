pub fn square_of_sum(n: u32) -> u32 {
    let s = (0..=n).sum::<u32>();
    s * s
}

pub fn sum_of_squares(n: u32) -> u32 {
    (0..=n).map(|x| x * x).sum::<u32>()
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
