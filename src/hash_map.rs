#[allow(unused_imports)]
use std::collections::HashMap;

#[macro_export]
macro_rules! hash_map {
    ($($key: expr => $value: expr),*) => {{
        const COUNT: usize = $crate::count![$($key),*];
        #[allow(unused_mut)]
        let mut hm = HashMap::with_capacity(COUNT);
        $(hm.entry($key).or_insert($value);)*
        hm
    }};
    ($($key: expr => $value: expr,)*) => {{
        $crate::hash_map!($($key => $value),*)
    }};
}

#[test]
fn empty_hash_map() {
    let hm: HashMap<(), ()> = hash_map![];
    assert!(hm.is_empty());
}

#[test]
fn single_pair() {
    let team: &'static str = "blue";
    let score = 64;
    let hm: HashMap<String, usize> = hash_map![team.to_string() => score];
    assert_eq!(hm.len(), 1);
    assert_eq!(hm[team], score);
}

#[test]
fn double_pair() {
    let team1: &'static str = "blue";
    let score1 = 64;
    let team2: &'static str = "black";
    let score2 = 66;
    let hm: HashMap<String, usize> = hash_map![
    team1.to_string() => score1,
    team2.to_string() => score2
    ];
    assert_eq!(hm.len(), 2);
    assert_eq!(hm[team1], score1);
    assert_eq!(hm[team2], score2)
}

#[test]
fn trailing() {
    let _ = hash_map![
    "team1".to_string() => 65,
    "team2".to_string() => 123,
    "team3".to_string() => 1,
    "team4".to_string() => 15,
    ];
}

#[test]
fn reapeated_no_overwrite() {
    let team1: &'static str = "blue";
    let score1 = 64;
    let team2: &'static str = "black";
    let score2 = 66;
    let hm: HashMap<String, usize> = hash_map![
    team1.to_string() => score1,
    team2.to_string() => score2,
    team1.to_string() => score2
    ];
    assert_eq!(hm.len(), 2);
    assert_eq!(hm[team1], score1);
    assert_eq!(hm[team2], score2)
}
