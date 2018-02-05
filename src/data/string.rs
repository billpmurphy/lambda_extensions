//! Church-encoded string represented as a pair-list of Church-encoded characters

use lambda_calculus::*;

use data::num::church;
use data::list::pair as pair_list;
use data::option;
use data::char;

/// Applied to a Church-encoded string it produces an `Option` containing the value of the string
/// parsed as an unsighed integer, represented as a Church-encoded numeral.
///
/// PARSE_NUM ≡ λs.IS_NUL s NONE (FOLDM (λac.AND_THEN (TO_DIGIT c) (λx.SOME (ADD x (MUL a TEN)))) ZERO s)
///           ≡ λ IS_NUL 1 NONE (FOLDM (λ λ AND_THEN (TO_DIGIT 1) (λ SOME (ADD 1 (MUL 3 TEN)))) ZERO 1)
///
/// # Example
/// ```
/// use lambda_extensions::*;
/// use lambda_extensions::data::string::parse_num;
///
/// assert_eq!(beta(app(parse_num(), "4".into_church()), NOR, 0), Some(4).into_church());
/// assert_eq!(beta(app(parse_num(), "10".into_church()), NOR, 0), Some(10).into_church());
/// ```
pub fn parse_num() -> Term {
    abs(app!(
        pair_list::is_nil(),
        Var(1),
        option::none(),
        app!(
            option::foldm(),
            abs!(2, app!(
                option::and_then(),
                app(char::to_digit(), Var(1)),
                abs(app(
                    option::some(),
                    app!(
                        church::add(),
                        Var(1),
                        app!(church::mul(), Var(3), 10.into_church())
                    )
                ))
            )),
            church::zero(),
            Var(1)
        )
    ))
}

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
