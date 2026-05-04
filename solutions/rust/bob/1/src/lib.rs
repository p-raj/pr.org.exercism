pub fn reply(message: &str) -> &str {
    let bobs_answers: Vec<&'static str>  = vec!["Sure.", "Whoa, chill out!", "Calm down, I know what I'm doing!", "Fine. Be that way!", "Whatever."];
    let end_with_q = message.trim().chars().last().unwrap_or(' ') == '?';
    let iter = message.chars().filter(|x| x.is_alphabetic());
    let yelled = iter
        .clone()
        .filter(|x| x.is_uppercase())
        .collect::<Vec<_>>()
        .len() > 0 &&
        iter
        .clone()
        .filter(|x| x.is_uppercase())
        .collect::<Vec<_>>()
        .len()
        == iter.clone().collect::<Vec<_>>().len();
    // silence
    let wspace = message
        .chars()
        .filter(|x| x.is_whitespace())
        .collect::<Vec<_>>()
        .len()
        == message.len();
    match (end_with_q, yelled, wspace) {
        (true, false, false) => bobs_answers[0],
        (false, true, false) => bobs_answers[1],
        (true, true, false) => bobs_answers[2],
        (false, false, true) => bobs_answers[3],
        _ => bobs_answers[4]
    }
}
