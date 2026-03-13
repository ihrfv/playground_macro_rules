#[allow(unused_imports)]
use std::collections::BTreeMap;

#[macro_export]
macro_rules! btree_map {
    ($($key: expr => $value: expr),*) => {{
        #[allow(unused_mut)]
        let mut btm = BTreeMap::new();
        $(btm.insert($key, $value);)*
        btm
    }};
    // trailing comma case
    ($($key: expr => $value: expr,)*) => {{
        $crate::btree_map!($($key => $value),*)
    }};
}

#[test]
fn empty() {
    let btm: BTreeMap<(), ()> = btree_map![];
    assert!(btm.is_empty());
}

#[test]
fn single_pair() {
    let team: &'static str = "blue";
    let score = 64;
    let hm = btree_map![team => score];
    assert_eq!(hm.len(), 1);
    assert_eq!(hm[team], score);
}

#[test]
fn double_pair() {
    let team1: &'static str = "blue";
    let score1 = 64;
    let team2: &'static str = "black";
    let score2 = 66;
    let hm = btree_map![
    team1 => score1,
    team2 => score2
    ];
    assert_eq!(hm.len(), 2);
    assert_eq!(hm[team1], score1);
    assert_eq!(hm[team2], score2)
}

#[test]
fn trailing() {
    let _ = btree_map![
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
    let hm = btree_map![
    team1 => score1,
    team2 => score2,
    team1 => score2
    ];
    assert_eq!(hm.len(), 2);
    assert_eq!(hm[team1], score1);
    assert_eq!(hm[team2], score2)
}
