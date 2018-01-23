//! [Single-pair list](https://en.wikipedia.org/wiki/Church_encoding#One_pair_as_a_list_node)

use lambda_calculus::*;
use lambda_calculus::data::option;
use lambda_calculus::data::pair;
use lambda_calculus::data::list::pair as pair_list;

/// Applied to a pair-encoded list it returns its first element; equivalent to `pair::fst`.
///
/// HEAD ≡ λp.(NIL p) NONE (SOME (p TRUE)) ≡ λ 1 TRUE ≡ FST
///
/// # Example
/// ```
/// use lambda_extensions::*;
/// use lambda_extensions::data::list::pair::safe_head;
///
/// let list1 = vec![4.into_church(), 5.into_church(), 6.into_church()].into_pair_list();
/// let list2 = vec![].into_pair_list();
///
/// let head1: Option<usize> = Some(4);
/// let head2: Option<usize> = None;
///
/// assert_eq!(beta(app(safe_head(), list1), NOR, 0), head1.into_church());
/// assert_eq!(beta(app(safe_head(), list2), NOR, 0), head2.into_church());
/// ```
pub fn safe_head() -> Term {
    abs(app!(
        app(pair_list::is_nil(), Var(1)),
        option::none(),
        app(option::some(), app(pair::fst(), Var(1)))
    ))
}
