pub fn build_proverb(list: &[&str]) -> String {
    if list.len() == 0 {
        return String::new();
    }
    let template = "For want of a {item1} the {item2} was lost.\n";
    let mut i = 0;
    let mut pvb = String::new();
    while i < list.len() - 1 {
        pvb.push_str(
            template
                .replace("{item1}", list[i])
                .replace("{item2}", list[i + 1])
                .as_str(),
        );
        i += 1;
    }
    pvb.push_str(
        "And all for the want of a {item}."
            .replace("{item}", list[0])
            .as_str(),
    );
    pvb
}
