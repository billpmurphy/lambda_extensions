//! Unicode character represented as a Church numeral, similar to
//! [Haskell](https://hackage.haskell.org/package/base-4.10.1.0/docs/Data-Char.html)

use lambda_calculus::*;
use lambda_calculus::data::num::church;
use lambda_calculus::data::boolean;

/// Applied to a lambda-encoded char, it returns a lambda-encoded boolean indicating whether the
/// char is in the ASCII character set.
///
/// IS_ASCII ≡ λx.LEQ x CHURCH_127 ≡ λ LEQ 1 CHURCH_127
///
/// # Examples
/// ```
/// use lambda_extensions::*;
/// use lambda_extensions::data::char::is_ascii;
///
/// assert_eq!(beta(app(is_ascii(), 'f'.into_church()), NOR, 0), true.into());
/// assert_eq!(beta(app(is_ascii(), 'é'.into_church()), NOR, 0), false.into());
/// ```
pub fn is_ascii() -> Term {
    abs(app!(church::leq(), Var(1), 127.into_church()))
}

/// Applied to a lambda-encoded char, it returns a lambda-encoded boolean indicating whether the
/// char is in the ASCII character set.
///
/// IS_DIGIT ≡ λx.AND (LEQ CHURCH_48 x) (LEQ x CHURCH_57) ≡ λ AND (LEQ CHURCH_48 x) (LEQ x CHURCH_57)
///
/// # Examples
/// ```
/// use lambda_extensions::*;
/// use lambda_extensions::data::char::is_digit;
///
/// assert_eq!(beta(app(is_digit(), 'a'.into_church()), NOR, 0), false.into());
/// assert_eq!(beta(app(is_digit(), '3'.into_church()), NOR, 0), true.into());
/// ```
pub fn is_digit() -> Term {
    abs(app!(
        boolean::and(),
        app!(church::leq(), 48.into_church(), Var(1)),
        app!(church::leq(), Var(1), 57.into_church())
    ))
}

/// Conversion from a Rust `char` to a Church-encoded character.
pub trait IntoChurchChar {
    /// Performs the conversion.
    fn into_church(self) -> Term;
}

impl IntoChurchChar for char {
    fn into_church(self) -> Term {
        (self as usize).into_church()
    }
}
