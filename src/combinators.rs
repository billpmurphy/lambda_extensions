//! Combinators and associated utilities

use lambda_calculus::*;
pub use lambda_calculus::combinators::*;

/// Returns `true` if the `Term` is one of the standard fixed-point combinators (Y, Z, or Turing's
/// combinator) and `false` otherwise.
///
/// ```
/// use lambda_extensions::*;
/// use lambda_extensions::combinators::is_fp_combinator;
///
/// let y = abs(app(
///     abs(app(Var(2), app(Var(1), Var(1)))),
///     abs(app(Var(2), app(Var(1), Var(1))))
/// ));
/// let omega = app(
///     abs(app(Var(1), Var(1))),
///     abs(app(Var(1), Var(1)))
/// );
///
/// assert!(is_fp_combinator(&y));
/// assert!(!is_fp_combinator(&omega));
/// ```
pub fn is_fp_combinator(term: &Term) -> bool {
    *term == Z() || *term == Y() || *term == T()
}

/// Returns `true` if the `Term` contains one of the standard fixed-point combinators (Y, Z, or
/// Turing's combinator) and `false` otherwise.
pub fn contains_fp_combinator(term: &Term) -> bool {
    match term {
        x if is_fp_combinator(x) => true,
        &App(ref f, ref a) => contains_fp_combinator(f) || contains_fp_combinator(a),
        &Abs(ref a) => contains_fp_combinator(a),
        &Var(_) => false,
    }
}
