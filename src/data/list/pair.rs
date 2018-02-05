//! [Single-pair list](https://en.wikipedia.org/wiki/Church_encoding#One_pair_as_a_list_node)

use lambda_calculus::*;
use lambda_calculus::combinators::{I, Z};

use lambda_calculus::data::num::church;
use lambda_calculus::data::pair;

use data::option;

pub use lambda_calculus::data::list::pair::*;

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
        app(is_nil(), Var(1)),
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
        is_nil(),
        Var(1),
        option::none(),
        app(
            option::some(),
            app!(
                pair::pair(),
                app(head(), Var(1)),
                app(tail(), Var(1))
            )
        )
    ))
}

/// Applied to a bit array represented as a pair-encoded list of lambda-encoded booleans, it
/// returns the numerical value of the bit array.
///
/// bin_to_cnum ≡ Z (λzax.(IS_NIL x) (λy.a) (λy.z (ADD (MUL (SUCC ONE) a) (HEAD x ONE ZERO)) (TAIL x)) I) ZERO
///             ≡ Z (λ λ λ (IS_NIL 1) (λ 3) (λ 4 (ADD (MUL (SUCC ONE) 3) (HEAD 2 ONE ZERO)) (TAIL 2)) I) ZERO
///
/// # Example
/// ```
/// use lambda_extensions::*;
/// use lambda_extensions::data::list::pair::bin_to_cnum;
///
/// let list1 = vec![true.into(), false.into()].into_pair_list();
/// let list2 = vec![true.into(), true.into()].into_pair_list();
///
/// assert_eq!(beta(app(bin_to_cnum(), list1), NOR, 0), 2.into_church());
/// assert_eq!(beta(app(bin_to_cnum(), list2), NOR, 0), 3.into_church());
/// ```
pub fn bin_to_cnum() -> Term {
    app!(
        Z(),
        abs!(3, app!(
            is_nil(),
            Var(1),
            abs(Var(3)),
            abs(app!(
                Var(4),
                app!(
                    church::add(),
                    app!(
                        church::mul(),
                        app(church::succ(), church::one()),
                        Var(3)
                    ),
                    app!(
                        head(),
                        Var(2),
                        church::one(),
                        church::zero()
                    )
                ),
                app(tail(), Var(2))
            )),
            I()
        )),
        church::zero()
    )
}
