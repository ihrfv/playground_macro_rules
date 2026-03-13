#[allow(unused_imports)]
use std::collections::HashMap;

#[macro_export]
macro_rules! hash_map {
    ($($key: expr => $value: expr),*) => {{
        const COUNT: usize = $crate::count![$($key),*];
        #[allow(unused_mut)]
        let mut hm = HashMap::with_capacity(COUNT);
        $(let _ = hm.entry($key).or_insert($value);)*
        hm
    }};
    // trailing comma case
    ($($key: expr => $value: expr,)*) => {{
        $crate::hash_map!($($key => $value),*)
    }};
}

#[test]
fn empty() {
    let hm: HashMap<(), ()> = hash_map![];
    assert!(hm.is_empty());
}

#[test]
fn single_pair() {
    let team: &'static str = "blue";
    let score = 64;
    let hm = hash_map![team => score];
    assert_eq!(hm.len(), 1);
    assert_eq!(hm[team], score);
}

#[test]
fn double_pair() {
    let team1: &'static str = "blue";
    let score1 = 64;
    let team2: &'static str = "black";
    let score2 = 66;
    let hm = hash_map![
    team1 => score1,
    team2 => score2
    ];
    assert_eq!(hm.len(), 2);
    assert_eq!(hm[team1], score1);
    assert_eq!(hm[team2], score2)
}

#[test]
fn trailing() {
    let _ = hash_map![
    "team1" => 65,
    "team2" => 123,
    "team3" => 1,
    "team4" => 15,
    ];
}

#[test]
fn reapeated_no_overwrite() {
    let team1: &'static str = "blue";
    let score1 = 64;
    let team2: &'static str = "black";
    let score2 = 66;
    let hm = hash_map![
    team1 => score1,
    team2 => score2,
    team1 => score2
    ];
    assert_eq!(hm.len(), 2);
    assert_eq!(hm[team1], score1);
    assert_eq!(hm[team2], score2)
}
