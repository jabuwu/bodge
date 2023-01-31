#[cfg(any(feature = "bodge_assert"))]
macro_rules! bodge_assert {
    ($($arg:tt)*) => ( assert!($($arg)*); )
}
#[cfg(not(any(feature = "bodge_assert")))]
macro_rules! bodge_assert {
    ($($arg:tt)*) => {};
}
