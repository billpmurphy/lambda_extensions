//! Conversion traits for lambda-encoded options

use lambda_calculus::*;

use combinators::{I, Z};
use data::convert::{TryFromTerm, TryFromTermChurch};
use data::list::pair as pair_list;

pub use lambda_calculus::data::option::*;

/// Applied to a function, a starting value and a pair-encoded list it performs a monadic
/// left fold on the list.
///
/// FOLDM :: (b -> a -> Option<b>) -> Option<b> -> List<a> -> Option<b>
///
/// FOLDM ≡ (λfal.Z (λzfsl.IS_NIL l (λx.s) (λx.z f (AND_THEN s (λx.f x (HEAD l))) (TAIL l)) I) f (SOME a) l
///
/// # Example
/// ```
/// use lambda_extensions::*;
/// use lambda_extensions::data::num::church::add;
/// use lambda_extensions::data::option::{some, foldm};
///
/// //assert_eq!(beta(app(is_ascii(), 'f'.into_church()), HAP, 0), true.into());
///
/// let empty = vec![].into_pair_list();
/// let list = vec![1.into_church(), 2.into_church(), 3.into_church()].into_pair_list();
/// let f = || abs!(2, app(some(), app!(add(), Var(1), Var(2))));
///
/// assert_eq!(beta(app!(foldm(), f(), 0.into_church(), empty), NOR, 0), Some(0).into_church());
/// assert_eq!(beta(app!(foldm(), f(), 0.into_church(), list), NOR, 0), Some(6).into_church());
/// ```
pub fn foldm() -> Term {
    abs!(3, app!(
        Z(),
        abs!(4, app!(
            pair_list::is_nil(),
            Var(1),
            abs(Var(3)),
            abs(app!(
                Var(5),
                Var(4),
                app!(
                    and_then(),
                    Var(3),
                    abs(app!(
                        Var(5),
                        Var(1),
                        app(pair_list::head(), Var(3))
                    ))
                ),
                app(pair_list::tail(), Var(2))
            )),
            I()
        )),
        Var(3),
        app(some(), Var(2)),
        Var(1)
    ))
}

macro_rules! make_option_trait {
    ($trait_name:ident, $impl_fn:ident, $type_name:ident) => (
        impl $trait_name<Option<$type_name>> for Term {
            fn $impl_fn(&self) -> Option<Option<$type_name>> {
                match self.unabs_ref().and_then(|x| x.unabs_ref()) {
                    Ok(&App(ref f, ref v)) if Ok(&1) == f.unvar_ref() => v.$impl_fn().map(Some),
                    Ok(&Var(2)) => Some(None),
                    _ => None,
                }
            }
        }
    );
}

make_option_trait!(TryFromTerm, try_from, bool);
make_option_trait!(TryFromTermChurch, try_from_church, bool);
make_option_trait!(TryFromTermChurch, try_from_church, char);
make_option_trait!(TryFromTermChurch, try_from_church, u64);
