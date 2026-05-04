use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples: HashSet<u32> = HashSet::new();
    for factor in factors {
        let f = *factor;
        if f == 0 {
            continue
        }
        let mut i = limit/f;
    println!("{}, {}", f*i, i);
        while i > 0 {
            if (f*i) < limit {multiples.insert(f*i);}
            i -= 1;
        }
    }
    multiples.into_iter().sum()
}
