//! Church-encoded string represented as a pair-list of Church-encoded characters

use lambda_calculus::*;

/// Conversion from a Rust `&str` to a Church-encoded character.
pub trait IntoChurchString {
    /// Performs the conversion.
    fn into_church(self) -> Term;
}

impl<'a> IntoChurchString for &'a str {
    fn into_church(self) -> Term {
        self.chars()
            .map(|c: char| (c as usize).into_church())
            .collect::<Vec<Term>>()
            .into_pair_list()
    }
}
