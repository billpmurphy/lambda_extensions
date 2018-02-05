//! Miscellaneous utilities

use lambda_calculus::*;

/// Assert that two `Term`s are equal when beta-reduced using any reduction `Order`.
/// Useful for testing.
pub fn assert_lc<A: Into<Term>, B: Into<Term>>(a: A, b: B) {
    let a_term = a.into();
    let b_term = b.into();
    assert_eq!(beta(a_term.clone(), NOR, 0), beta(b_term.clone(), NOR, 0));
    assert_eq!(beta(a_term.clone(), HNO, 0), beta(b_term.clone(), HNO, 0));
    assert_eq!(beta(a_term.clone(), HAP, 0), beta(b_term.clone(), HAP, 0));
}
