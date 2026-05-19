use std::collections::HashMap;
use std::sync::LazyLock;

static PLANTS_MAP: LazyLock<HashMap<u8, &str>> = LazyLock::new(|| {
    HashMap::from([
        (b'C', "clover"),
        (b'G', "grass"),
        (b'R', "radishes"),
        (b'V', "violets"),
    ])
});

static STUDENTS_MAP: LazyLock<HashMap<&str, usize>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    let items = [
        "Alice",
        "Bob",
        "Charlie",
        "David",
        "Eve",
        "Fred",
        "Ginny",
        "Harriet",
        "Ileana",
        "Joseph",
        "Kincaid",
        "Larry"
    ];
    items.iter().copied().enumerate().for_each(|(i, name)| {
        map.insert(name, i);
    });
    map
});

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let student_idx = STUDENTS_MAP.get(&student);
    if student_idx.is_none() {
        panic!("Student not found");
    }
    let student_idx = *student_idx.unwrap() * 2;

    let diagram_width = diagram.as_bytes()
            .iter()
            .enumerate()
            .filter_map(|(i,&c)| if c == b'\n' {Some(i)} else {None})
            .next()
            .unwrap()
    ;

    let mut student_plants = vec![
        diagram.as_bytes()[student_idx],
        diagram.as_bytes()[student_idx+1],
        diagram.as_bytes()[diagram_width + student_idx+1],
        diagram.as_bytes()[diagram_width + student_idx+2],
    ];

    student_plants.iter().copied().map(|p| *PLANTS_MAP.get(&p).unwrap()).collect()
}
