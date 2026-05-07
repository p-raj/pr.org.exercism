pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = vec![];
    for c in string.chars() {
        match c {
            '[' | '{' | '(' => stack.push(c),
            ']' | '}' | ')' => {
                if stack.is_empty() {
                    return false
                } else {
                    if let Some(prev) = stack.pop() {
                    match (prev, c) {
                        ('[', ']') | ('{', '}') | ('(', ')') => (),
                        _ => return false
                    }
                    } else {
                        return false
                    }
                }
            }
            _ => (),
        }
    }
    stack.is_empty()
}
