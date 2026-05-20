use std::{collections::HashMap, sync::LazyLock};

static PLANTS_MAP: LazyLock<HashMap<u8, &str>> = LazyLock::new(|| HashMap::from([
    (b'C', "clover"),
    (b'G', "grass"),
    (b'R', "radishes"),
    (b'V', "violets"),
]));

static STUDENT_INDEXES: LazyLock<HashMap<&str, usize>> = LazyLock::new(|| {
    let names = "Alice, Bob, Charlie, David, Eve, Fred, Ginny, Harriet, Ileana, Joseph, Kincaid, Larry";
    let mut indexes = HashMap::new();
    names.split(", ").enumerate().for_each(|(i, name)| {
        indexes.insert(name, i);
    });
    indexes
});

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let (top, bottom) = diagram
        .split_once('\n')
        .expect("Invalid diagram text, as it can't be divided.");
    let top = top.as_bytes();
    let bottom = bottom.as_bytes();
    
    let index = STUDENT_INDEXES[student];
    let offset = index * 2;
    
    vec![
        PLANTS_MAP[&top[offset]],
        PLANTS_MAP[&top[offset + 1]],
        PLANTS_MAP[&bottom[offset]],
        PLANTS_MAP[&bottom[offset + 1]],
    ]
}
