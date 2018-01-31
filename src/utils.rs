//! Miscellaneous utilities

use lambda_calculus::*;
use lambda_calculus::combinators::{Z, Y, T};

/// Assert that two `Term` are equal when beta-reduced using any reduction `Order`.
/// Useful for testing.
pub fn assert_lc<A: Into<Term>, B: Into<Term>>(a: A, b: B) {
    let a_term = a.into();
    let b_term = b.into();
    assert_eq!(beta(a_term.clone(), NOR, 0), beta(b_term.clone(), NOR, 0));
    assert_eq!(beta(a_term.clone(), HNO, 0), beta(b_term.clone(), HNO, 0));
    assert_eq!(beta(a_term.clone(), HAP, 0), beta(b_term.clone(), HAP, 0));
}

/// Returns `true` if the `Term` is one of the standard fixed-point combinators (Y, Z, or Turing's
/// combinator) and `false` otherwise.
pub fn is_fp_combinator(term: &Term) -> bool {
    *term == Z() || *term == Y() || *term == T()
}
