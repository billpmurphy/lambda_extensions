//! Lambda-encoded result

use lambda_calculus::*;

use data::monad::make_foldm;

pub use lambda_calculus::data::result::*;

/// Applied to a function that takes two arguments and returns an `Result`, a starting value, and a
/// pair-encoded list it performs a monadic left fold on the list, returning an `Result`.
///
/// FOLDM :: (b -> a -> Result b) -> b -> [a] -> Result b
///
/// # Example
/// ```
/// use lambda_extensions::*;
/// use lambda_extensions::data::num::church::add;
/// use lambda_extensions::data::result::{ok, foldm};
///
/// let empty = vec![].into_pair_list();
/// let list = vec![1.into_church(), 2.into_church(), 3.into_church()].into_pair_list();
/// let f = || abs!(2, app(ok(), app!(add(), Var(1), Var(2))));
///
/// let result: Result<usize, usize> = Ok(6);
/// assert_eq!(beta(app!(foldm(), f(), 0.into_church(), list), NOR, 0), result.into_church());
/// ```
pub fn foldm() -> Term {
    make_foldm(and_then(), ok())
}
