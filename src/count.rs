#[macro_export]
#[doc(hidden)]
macro_rules! count {
    ($($element:expr),*) => {
        <[()]>::len(&[$(crate::count!(@SUBST; $element)),*])
    };
    (@SUBST; $_element:expr) => {
        ()
    };
}
