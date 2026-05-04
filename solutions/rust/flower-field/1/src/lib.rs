pub fn annotate(garden: &[&str]) -> Vec<String> {
    // todo!(
    //     "\nAnnotate each square of the given garden with the number of flowers that surround said square (blank if there are no surrounding flowers):\n{garden:#?}\n"
    // );
    let rows = garden.len();
    let cols = if garden.len() > 0 {garden[0].len()} else {0};
    let mut res: Vec<String> = Vec::new();
    let mut r = 0;
    while r < rows {
        let mut c = 0;
        let mut s = String::new();
        while c < cols {
            match garden[r].as_bytes()[c] as char {
                ' ' => {
                    let mut flowers = 0;
                    if r >= 1 && garden[r-1].as_bytes()[c] as char == '*' {
                        flowers += 1
                    }
                    if r+1 < rows && garden[r+1].as_bytes()[c] as char == '*' {
                        flowers += 1
                    }
                    if c >= 1 && garden[r].as_bytes()[c-1] as char == '*' {
                        flowers += 1
                    }
                    if c + 1 < cols && garden[r].as_bytes()[c+1] as char == '*' {
                        flowers += 1
                    }
                    if r >= 1 && c >= 1 && garden[r-1].as_bytes()[c-1] as char == '*' {
                        flowers += 1
                    }
                    if r+1 < rows && c+1 < cols && garden[r+1].as_bytes()[c+1] as char == '*' {
                        flowers += 1
                    }
                    if r >= 1 && c+1 < cols && garden[r-1].as_bytes()[c+1] as char == '*' {
                        flowers += 1
                    }
                    if r+1 < rows && c >= 1 && garden[r+1].as_bytes()[c-1] as char == '*' {
                        flowers += 1
                    }
                    if flowers > 0{
                        s.push_str(flowers.to_string().as_str());   
                    } else {
                        s.push_str(" ")
                    }
                },
                '*' => {
                    s.push_str("*");
                },
                _ => println!("ERROR"),
            };
            c += 1;
        }
        res.push(s);
        r += 1;
    }
    res
}
