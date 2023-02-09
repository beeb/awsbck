/// Utility traits
use std::fmt;

/// An alias for unwrap when the code has been audited to ensure that the value is not None/Err or when panic
/// is required.
pub(crate) trait OrPanic<T> {
    /// An alias for unwrap when the code has been audited to ensure that the value is not None/Err or when panic
    /// is required.
    fn or_panic(self) -> T;
}

#[allow(clippy::unwrap_used)]
impl<T> OrPanic<T> for Option<T> {
    fn or_panic(self) -> T {
        self.unwrap()
    }
}

#[allow(clippy::unwrap_used)]
impl<T, E> OrPanic<T> for Result<T, E>
where
    E: fmt::Debug,
{
    fn or_panic(self) -> T {
        self.unwrap()
    }
}
