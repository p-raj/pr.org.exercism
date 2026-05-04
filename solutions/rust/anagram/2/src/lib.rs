use std::collections::HashSet;
use std::collections::HashMap;

fn freq_counter(word: &str) -> HashMap<char, usize> {
    let mut wc: HashMap<char, usize> = HashMap::new();
    for ch in word.to_lowercase().chars() {
        match wc.get_mut(&ch) {
            Some(count) => *count += 1,
            None => {wc.insert(ch, 1);},
        }
    }
    wc
}

fn hash_match(hmap: &HashMap<char, usize>, other_hmap: &HashMap<char, usize>) -> bool {
    let mut matches = true;
    if hmap.len() != other_hmap.len() {
        return false;
    }
    for (k, v) in hmap.iter() {
        match other_hmap.get(k) {
            Some(count) => matches &= v == count,
            None => return false
        }
    }
    matches
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut res: HashSet<&'a str> = HashSet::new();
    let word_wc = freq_counter(word);
    for pana in possible_anagrams {
        let ana_wc = freq_counter(pana);
        if hash_match(&word_wc, &ana_wc) && (pana.to_lowercase() != word.to_lowercase()) {
            res.insert(pana);
        }
    }
    res
}
