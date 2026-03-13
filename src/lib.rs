#[macro_export]
macro_rules! avec {
    ($($element:expr),* $(,)?) => {{
        #[allow(unused_mut)]
        let mut v = Vec::new();
        $(v.push($element);)*
        v
    }};
    ($element: expr; $count: expr) => {{
        let count = $count;
        let mut v = Vec::with_capacity(count);
        v.resize(count, $element);
        v
    }}
}

#[test]
fn empty_vector() {
    let v: Vec<()> = avec![];
    assert!(v.is_empty());
}

#[test]
fn single() {
    let v = avec![42];
    assert!(!v.is_empty());
    assert_eq!(v.len(), 1);
    assert_eq!(v[0], 42);
}

#[test]
fn double() {
    let v = avec![42, 43];
    assert!(!v.is_empty());
    assert_eq!(v.len(), 2);
    assert_eq!(v[0], 42);
    assert_eq!(v[1], 43);
}

#[test]
fn with_capacity() {
    let v = avec![42; 5];
    assert!(!v.is_empty());
    assert_eq!(v.len(), 5);
    for e in &v {
        assert_eq!(*e, 42);
    }
}

#[test]
fn trailing() {
    let _ = avec![
        "akjlashdfpbnawpoefibnpoqweinfoinasdionaospdnivaosnfd",
        "akjlashdfpbnawpoefibnpoqweinfoinasdionaospdnivaosnfd",
        "akjlashdfpbnawpoefibnpoqweinfoinasdionaospdnivaosnfd",
        "akjlashdfpbnawpoefibnpoqweinfoinasdionaospdnivaosnfd",
        "akjlashdfpbnawpoefibnpoqweinfoinasdionaospdnivaosnfd",
    ];
}

/// ```compile_fail
/// let x: Vec<u32> = playground_macro_rules::avec![42; "should have been a usize here"];
/// ```
#[allow(dead_code)]
struct CompilationFailureTest;