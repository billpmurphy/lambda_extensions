//! Unicode character represented as a Church numeral, similar to
//! [Haskell](https://hackage.haskell.org/package/base-4.10.1.0/docs/Data-Char.html)

use lambda_calculus::*;
use lambda_calculus::data::num::church;
use lambda_calculus::data::boolean;
use lambda_calculus::data::option;
use data::convert::TryFromTermChurch;

use std::char::{MAX, from_u32};

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
/// assert_eq!(beta(app(is_ascii(), 'f'.into_church()), HAP, 0), true.into());
/// assert_eq!(beta(app(is_ascii(), 'é'.into_church()), HAP, 0), false.into());
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
/// assert_eq!(beta(app(is_ascii_digit(), 'a'.into_church()), HAP, 0), false.into());
/// assert_eq!(beta(app(is_ascii_digit(), '3'.into_church()), HAP, 0), true.into());
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
/// assert_eq!(beta(app(is_ascii_alpha(), 'B'.into_church()), HAP, 0), true.into());
/// assert_eq!(beta(app(is_ascii_alpha(), '3'.into_church()), HAP, 0), false.into());
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
/// assert_eq!(beta(app(is_ascii_whitespace(), '\n'.into_church()), HAP, 0), true.into());
/// assert_eq!(beta(app(is_ascii_whitespace(), '3'.into_church()), HAP, 0), false.into());
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

/// Applied to a lambda-encoded character, it returns a lambda-encoded boolean indicating whether
/// the character is an ASCII uppercase letter (A-Z).
///
/// IS_ASCII_UPPER ≡ λx.AND (LEQ `'A'` x) (LEQ x `'Z'`)) ≡ λ AND (LEQ `'A'` 1) (LEQ 1 `'Z'`)
///
/// # Examples
/// ```
/// use lambda_extensions::*;
/// use lambda_extensions::data::char::is_ascii_upper;
///
/// assert_eq!(beta(app(is_ascii_upper(), 'F'.into_church()), HAP, 0), true.into());
/// assert_eq!(beta(app(is_ascii_upper(), 'f'.into_church()), HAP, 0), false.into());
/// ```
pub fn is_ascii_upper() -> Term {
    abs(app!(
        boolean::and(),
        app!(church::leq(), 'A'.into_church(), Var(1)),
        app!(church::leq(), Var(1), 'Z'.into_church())
    ))
}

/// Applied to a lambda-encoded character, it returns a lambda-encoded boolean indicating whether
/// the character is an ASCII lowercase letter (a-z).
///
/// IS_ASCII_LOWER ≡ λx.AND (LEQ `'a'` x) (LEQ x `'z'`)) ≡ λ AND (LEQ `'a'` 1) (LEQ 1 `'z'`)
///
/// # Examples
/// ```
/// use lambda_extensions::*;
/// use lambda_extensions::data::char::is_ascii_lower;
///
/// assert_eq!(beta(app(is_ascii_lower(), 'F'.into_church()), HAP, 0), false.into());
/// assert_eq!(beta(app(is_ascii_lower(), 'f'.into_church()), HAP, 0), true.into());
/// ```
pub fn is_ascii_lower() -> Term {
    abs(app!(
        boolean::and(),
        app!(church::leq(), 'a'.into_church(), Var(1)),
        app!(church::leq(), Var(1), 'z'.into_church())
    ))
}

/// Applied to a lambda-encoded character, if the character is an ASCII lowercase letter (a-z) it
/// returns the equivalent ASCII uppercase letter (A-Z), otherwise it returns the character.
///
/// TO_ASCII_UPPER ≡ λx.(IS_ASCII_LOWER x) (SUB x `32`) x ≡ λ (IS_ASCII_LOWER 1) (SUB 1 `32`) 1
///
/// # Examples
/// ```
/// use lambda_extensions::*;
/// use lambda_extensions::data::char::to_ascii_upper;
///
/// assert_eq!(beta(app(to_ascii_upper(), '3'.into_church()), HAP, 0), '3'.into_church());
/// assert_eq!(beta(app(to_ascii_upper(), 'f'.into_church()), HAP, 0), 'F'.into_church());
/// ```
pub fn to_ascii_upper() -> Term {
    abs(app!(
        app(is_ascii_lower(), Var(1)),
        app!(church::sub(), Var(1), 32.into_church()),
        Var(1)
    ))
}

/// Applied to a lambda-encoded character, if the character is an ASCII uppercase letter (A-Z) it
/// returns the equivalent ASCII lowercase letter (a-z), otherwise it returns the character.
///
/// TO_ASCII_LOWER ≡ λx.(IS_ASCII_UPPER x) (ADD x `32`) x ≡ λ (IS_ASCII_UPPER 1) (ADD 1 `32`) 1
///
/// # Examples
/// ```
/// use lambda_extensions::*;
/// use lambda_extensions::data::char::to_ascii_lower;
///
/// assert_eq!(beta(app(to_ascii_lower(), '3'.into_church()), HAP, 0), '3'.into_church());
/// assert_eq!(beta(app(to_ascii_lower(), 'F'.into_church()), HAP, 0), 'f'.into_church());
/// ```
pub fn to_ascii_lower() -> Term {
    abs(app!(
        app(is_ascii_upper(), Var(1)),
        app!(church::add(), Var(1), 32.into_church()),
        Var(1)
    ))
}

/// Applied to a lambda-encoded character, it returns a lambda-encoded `Option` containing the
/// character converted to a Church numeral, if the character is an ASCII digit (0-9).
///
/// TO_DIGIT ≡ λx.(IS_ASCII_DIGIT x) (SOME (SUB x `48`)) NONE ≡
/// λ (IS_ASCII_DIGIT 1) (SOME (SUB 1 `48`)) NONE
///
/// # Examples
/// ```
/// use lambda_extensions::*;
/// use lambda_extensions::data::char::to_digit;
///
/// let none: Option<usize> = None;
/// assert_eq!(beta(app(to_digit(), '3'.into_church()), HAP, 0), Some(3).into_church());
/// assert_eq!(beta(app(to_digit(), 'a'.into_church()), HAP, 0), none.into_church());
/// ```
pub fn to_digit() -> Term {
    abs(app!(
        app(is_ascii_digit(), Var(1)),
        app(option::some(), app!(church::sub(), Var(1), 48.into_church())),
        option::none()
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

impl TryFromTermChurch<char> for Term {
    fn try_from_church(&self) -> Option<char> {
        let mut inner: &Term = match self.unabs_ref().and_then(|x| x.unabs_ref()) {
            Ok(v) => v,
            _ => return None
        };

        let mut num: u32 = 0;
        while num <= MAX as u32 {
            match inner {
                &Var(1) => return from_u32(num),
                &App(ref f, ref a) if Ok(&2) == f.unvar_ref() => {
                    num += 1;
                    inner = a;
                }
                _ => break,
            }
        }
        None
    }
}
