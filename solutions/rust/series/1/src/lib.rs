pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    if digits.is_empty() {
        return res;
    }
    if len == 0 {
        return res;
    }
    if len > digits.len() {
        return res;
    }
    let mut idx = 0;
    while idx + len <= digits.len() {
        res.push(digits[idx..idx + len].to_string());
        idx += 1;
    }
    res
}
