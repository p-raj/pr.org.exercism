pub fn is_valid(code: &str) -> bool {
    let mut count = 0;

    let sum = match code
        .chars()
        .filter(|c| *c != ' ')
        .rev()
        .enumerate()
        .map(|(i, c)| {
            count += 1;
            c.to_digit(10).map(|d| {
                if i % 2 == 1 {
                    let dbl = d * 2;
                    if dbl > 9 { dbl - 9 } else { dbl }
                } else {
                    d
                }
            })
        })
        .collect::<Option<Vec<_>>>() // early exit if invalid
    {
        Some(v) if count > 1 => v.into_iter().sum::<u32>(),
        _ => return false,
    };

    sum % 10 == 0
}