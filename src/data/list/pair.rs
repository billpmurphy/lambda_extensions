//! [Single-pair list](https://en.wikipedia.org/wiki/Church_encoding#One_pair_as_a_list_node)

use lambda_calculus::*;
use lambda_calculus::combinators::Z;
use lambda_calculus::data::list::pair as pair_list;
use lambda_calculus::data::num::church;
use lambda_calculus::data::option;
use lambda_calculus::data::boolean;
use lambda_calculus::data::pair;

/// Applied to a pair-encoded list it returns an `Option` containing its first element.
///
/// SAFE_HEAD ≡ λp.(NIL p) NONE (SOME (FST p)) ≡ λ (NIL 1) NONE (SOME (FST 1))
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

/// Applied to a pair-encoded list it returns an `Option` containing
///
/// uncons ≡ λp.(NIL p) NONE (SOME (PAIR (HEAD p) (TAIL p)))
///        ≡ λp.(NIL p) NONE (SOME (PAIR (HEAD p) (TAIL p)))
///
/// # Example
/// ```
/// use lambda_extensions::*;
/// use lambda_extensions::data::list::pair::uncons;
///
/// let list1 = vec![4.into_church(), 3.into_church()].into_pair_list();
/// let list2 = vec![3.into_church()].into_pair_list();
/// let empty = vec![].into_pair_list();
///
/// let pair1: Term = (4.into_church(), list2.clone()).into();
/// let pair2: Term = (3.into_church(), empty.clone()).into();
/// let none: Option<usize> = None;
///
/// assert_eq!(beta(app(uncons(), list1.clone()), NOR, 0), Some(pair1).into());
/// assert_eq!(beta(app(uncons(), list2.clone()), NOR, 0), Some(pair2).into());
/// assert_eq!(beta(app(uncons(), empty.clone()), NOR, 0), none.into_church());
/// ```
pub fn uncons() -> Term {
    abs(app!(
        pair_list::is_nil(),
        Var(1),
        option::none(),
        app(
            option::some(),
            app!(
                pair::pair(),
                app(pair_list::head(), Var(1)),
                app(pair_list::tail(), Var(1))
            )
        )
    ))
}
