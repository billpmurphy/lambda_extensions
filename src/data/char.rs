//! Unicode character represented as a Church numeral, similar to
//! [Haskell](https://hackage.haskell.org/package/base-4.10.1.0/docs/Data-Char.html)

use lambda_calculus::*;
use lambda_calculus::data::num::church;
use lambda_calculus::data::boolean;

/// Applied to a lambda-encoded character, it returns a lambda-encoded boolean indicating whether
/// the character is in the ASCII character set.
///
/// IS_ASCII ≡ λx.LEQ x `'\x7F'` ≡ λ LEQ 1 `'\x7F'`
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
    abs(app!(church::leq(), Var(1), '\x7F'.into_church()))
}

/// Applied to a lambda-encoded character, it returns a lambda-encoded boolean indicating whether
/// the character is an ASCII digit (0-9).
///
/// IS_ASCII_DIGIT ≡ λx.AND (LEQ `'0'` x) (LEQ x `'9'`) ≡
/// λ AND (LEQ `'0'` x) (LEQ x `'9'`)
///
/// # Examples
/// ```
/// use lambda_extensions::*;
/// use lambda_extensions::data::char::is_ascii_digit;
///
/// assert_eq!(beta(app(is_ascii_digit(), 'a'.into_church()), NOR, 0), false.into());
/// assert_eq!(beta(app(is_ascii_digit(), '3'.into_church()), NOR, 0), true.into());
/// ```
pub fn is_ascii_digit() -> Term {
    abs(app!(
        boolean::and(),
        app!(church::leq(), '0'.into_church(), Var(1)),
        app!(church::leq(), Var(1), '9'.into_church())
    ))
}

/// Applied to a lambda-encoded character, it returns a lambda-encoded boolean indicating whether
/// the character is an ASCII letter (A-Z or a-z).
///
/// IS_ASCII_ALPHA ≡ λx.OR (AND (LEQ `'a'` x) (LEQ x `'z'`)) (AND (LEQ `'A'` x) (LEQ x `'Z'`)) ≡
/// λ OR (AND (LEQ `'a'` 1) (LEQ 1 `'z'`)) (AND (LEQ `'A'` 1) (LEQ 1 `'Z'`)) ≡
///
/// # Examples
/// ```
/// use lambda_extensions::*;
/// use lambda_extensions::data::char::is_ascii_alpha;
///
/// assert_eq!(beta(app(is_ascii_alpha(), 'B'.into_church()), NOR, 0), true.into());
/// assert_eq!(beta(app(is_ascii_alpha(), '3'.into_church()), NOR, 0), false.into());
/// ```
pub fn is_ascii_alpha() -> Term {
    abs(app!(
        boolean::or(),
        app!(
            boolean::and(),
            app!(church::leq(), 'a'.into_church(), Var(1)),
            app!(church::leq(), Var(1), 'z'.into_church())
        ),
        app!(
            boolean::and(),
            app!(church::leq(), 'A'.into_church(), Var(1)),
            app!(church::leq(), Var(1), 'Z'.into_church())
        )
    ))
}

/// Applied to a lambda-encoded character, it returns a lambda-encoded boolean indicating whether
/// the character is an [ASCII whitespace
/// character](https://infra.spec.whatwg.org/#ascii-whitespace): U+0009 TAB, U+000A LF, U+000C FF,
/// U+000D CR, or U+0020 SPACE.
///
/// IS_ASCII_WHITESPACE ≡ λx.OR (EQ `'\t'` x) (OR (EQ x `'\n'`) (OR (EQ x `'\x0C'`)
/// (OR (EQ x `'\r'`) (EQ x `' '`))))
/// ≡ λ OR (EQ `'\t'` 1) (OR (EQ 1 `'\n'`) (OR (EQ 1 `'\x0C'`) (OR (EQ 1 `'\r'`) (EQ 1 `' '`))))
///
/// # Examples
/// ```
/// use lambda_extensions::*;
/// use lambda_extensions::data::char::is_ascii_whitespace;
///
/// assert_eq!(beta(app(is_ascii_whitespace(), '\n'.into_church()), NOR, 0), true.into());
/// assert_eq!(beta(app(is_ascii_whitespace(), '3'.into_church()), NOR, 0), false.into());
/// ```
pub fn is_ascii_whitespace() -> Term {
    abs(app!(
        boolean::or(),
        app!(church::eq(), '\t'.into_church(), Var(1)),
        app!(
            boolean::or(),
            app!(church::eq(), '\n'.into_church(), Var(1)),
            app!(
                boolean::or(),
                app!(church::eq(), '\x0C'.into_church(), Var(1)),
                app!(
                    boolean::or(),
                    app!(church::eq(), '\r'.into_church(), Var(1)),
                    app!(church::eq(), ' '.into_church(), Var(1))
                )
            )
        )
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
