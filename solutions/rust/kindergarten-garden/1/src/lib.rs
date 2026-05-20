use std::iter::zip;
pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let students: Vec<&'static str> = vec![
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
        "Kincaid", "Larry",
    ];
    let drows: Vec<_> = diagram.split("\n").collect();
    let mut student_idx = 0;
    for (idx, stud) in students.iter().enumerate() {
        if *stud == student {
            student_idx = idx;
            break;
        }
    }
    let mut plants: Vec<&'static str> = Vec::new();
    let p1 = student_idx * 2;
    println!(
        "{:?}{:?}{:?}{:?}",
        drows[0],
        drows[1],
        drows[0].chars().nth(p1),
        drows[1].chars().nth(p1 + 1)
    );
    let mut zipper = zip([0,0,1,1], [p1, p1+1, p1, p1+1]);
    for (ridx, pidx) in zipper {
        match drows[ridx].chars().nth(pidx) {
        Some(x) => match x {
            'G' => plants.push("grass"),
            'C' => plants.push("clover"),
            'R' => plants.push("radishes"),
            'V' => plants.push("violets"),
            _ => {}
        },
        None => {}
    }
    }
    plants
}
