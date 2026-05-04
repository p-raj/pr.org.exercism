use std::collections::HashMap;

fn get_num(num: &u32) -> (String, bool) {
    let eng_num: HashMap<u32, (String, bool)> = HashMap::from([
    (10, (String::from("Ten"), true)),
    (9, (String::from("Nine"), true)),
    (8, (String::from("Eight"),true)),
    (7, (String::from("Seven"),true)),
    (6, (String::from("Six"),true)),
    (5, (String::from("Five"),true)),
    (4, (String::from("Four"),true)),
    (3, (String::from("Three"),true)),
    (2, (String::from("Two"),true)),
    (1, (String::from("One"),false)),
    (0, (String::from("no"),true)),
]);
    eng_num.get(num).unwrap().clone()
}

fn hydrate(bottle: u32) -> String {
    let template = String::from("
{num} green bottle{s} hanging on the wall,
{num} green bottle{s} hanging on the wall,
And if one green bottle should accidentally fall,
There'll be {down_one} green bottle{ds} hanging on the wall.
");
    let en = get_num(&bottle);
    let dn = get_num(&(bottle-1));
    let num = en.0.clone();
    let down_one = dn.0.clone().to_lowercase();
    let s = if en.1 {String::from("s")} else {String::from("")};
    let ds = if dn.1 {String::from("s")} else {String::from("")};
    template.replace("{num}", &num).replace("{down_one}", &down_one).replace("{s}", &s).replace("{ds}", &ds)

}
pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut res = String::new();
    let mut count = 0;
    let mut take_down = take_down;
    while take_down > 0 {
        res.push_str(hydrate(start_bottles-count).as_str());
        take_down -= 1;
        count += 1
    }
    res
}
