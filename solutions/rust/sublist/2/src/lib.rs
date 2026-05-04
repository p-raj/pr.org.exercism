#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list.is_empty() && second_list.is_empty() {
        return Comparison::Equal;
    } else if first_list.is_empty() && second_list.len() > 0 {
        return Comparison::Sublist;
    } else if first_list.len() > 0 && second_list.is_empty() {
        return Comparison::Superlist;
    }
    let mut idxa = 0;
    let mut idxb = 0;
    while idxa < first_list.len() {
        let mut frst_match = 0;
        let mut last_match = 0;
        while idxb < second_list.len() && second_list[idxb] != first_list[idxa] {
            idxb += 1;
        }
        if idxb == second_list.len() {
            idxa += 1;
            idxb = 0;
        } else {
            frst_match = idxb;
            last_match = frst_match;
            while idxa < first_list.len()
                && idxb < second_list.len()
                && second_list[idxb] == first_list[idxa]
            {
                idxa += 1;
                idxb += 1;
                last_match += 1;
            }
            let count_match = last_match - frst_match;
            if count_match == first_list.len() && count_match == second_list.len() {
                return Comparison::Equal;
            } else if count_match < first_list.len() && count_match == second_list.len() {
                return Comparison::Superlist;
            } else if count_match == first_list.len() && count_match < second_list.len() {
                return Comparison::Sublist;
            } else {
                idxb = frst_match + 1;
                idxa -= count_match;
            }
        }
    }
    Comparison::Unequal
}
