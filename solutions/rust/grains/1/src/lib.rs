pub fn square(s: u32) -> u64 {
    if s == 1 {
        return 1
    }
    square(s-1) * 2
}

pub fn total() -> u64 {
    let mut board = [0u64;64];
    for (idx, val) in board.into_iter().enumerate() {
        if val == 0 {
            board[idx] = square((idx + 1) as u32);
        }
    }
    board.iter().sum()
}
