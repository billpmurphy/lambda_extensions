//! Conversion traits

/// Attempted conversion from a `Term` to Rust type.
pub trait TryFromTerm<T> {
    /// Performs the conversion
    fn try_from(&self) -> Option<T>;
}

/// Attempted conversion from a Church-encoded datatype `Term` to Rust type.
pub trait TryFromTermChurch<T> {
    /// Performs the conversion
    fn try_from_church(&self) -> Option<T>;
}
