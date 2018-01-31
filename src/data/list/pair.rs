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

/*
>>>>>>> Stashed changes
/// Applied to a Church-encoded number `i` and a pair-encoded list it returns an `Option`
/// containing the `i`-th (zero-indexed) element of the list.
///
/// SAFE_INDEX ≡ λil.SAFE_HEAD (i TAIL l) ≡ λ λ SAFE_HEAD (2 TAIL 1)
///
/// # Example
/// ```
/// use lambda_extensions::data::list::pair::safe_index;
/// use lambda_extensions::*;
///
/// let list = || vec![1.into_church(), 2.into_church(), 3.into_church()].into_pair_list();
/// let none: Option<usize> = None;
///
/// assert_eq!(beta(app!(safe_index(), 0.into_church(), list()), NOR, 0), Some(1).into_church());
/// assert_eq!(beta(app!(safe_index(), 1.into_church(), list()), NOR, 0), Some(2).into_church());
/// assert_eq!(beta(app!(safe_index(), 3.into_church(), list()), NOR, 0), none.into_church());
/// ```
pub fn safe_index() -> Term {
    abs!(2, app(
        safe_head(),
        app!(
            Var(2),
            pair_list::tail(),
            Var(1)
        )
    ))
}
<<<<<<< Updated upstream
=======
/// Applied to a Church-encoded number `i` and a pair-encoded list it returns an `Option`
/// containing the `i`-th (zero-indexed) element of the list.
///
/// SAFE_INDEX ≡ λil.SAFE_HEAD (i TAIL l) ≡ λ λ SAFE_HEAD (2 TAIL 1)
///
/// # Example
/// ```
/// extern crate lambda_calculus;
/// use lambda_extensions::data::num::church;
/// use lambda_extensions::*;
/// use lambda_extensions::data::list::pair::elem;
///
/// let list = || vec![1.into_church(), 2.into_church(), 3.into_church()].into_pair_list();
///
/// assert_eq!(beta(app!(elem(church::eq()), 1.into_church(), list()), NOR, 0), true.into());
/// assert_eq!(beta(app!(elem(church::eq()), 4.into_church(), list()), NOR, 0), false.into());
/// assert_eq!(beta(app!(elem(church::eq()), 3.into_church(), list()), NOR, 0), true.into());
/// ```
pub fn elem(eq: Term) -> Term {
    app(
        Z(),
        abs!(3, app!(
            pair_list::is_nil(),
            Var(1),
            boolean::fls(),
            app!(
                boolean::or(),
                app!(
                    eq,
                    app(pair_list::head(), Var(1)),
                    Var(2)
                ),
                app!(
                    Var(3),
                    Var(2),
                    app(pair_list::tail(), Var(1))
                )
            )
        ))
    )
}
*/
