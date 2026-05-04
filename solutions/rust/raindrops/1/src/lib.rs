pub fn raindrops(n: u32) -> String {
    let mut sound = String::new();
    let mut n = n;
    if n % 3 == 0 {
        sound.push_str("Pling");
        n /= 3;
    }
    if n % 5 == 0 {
        sound.push_str("Plang");
        n /= 5;
    }
    if n % 7 == 0 {
        sound.push_str("Plong");
        n /= 7;
    }
    if sound.len() == 0 {
        return n.to_string();
    }
    sound
}
