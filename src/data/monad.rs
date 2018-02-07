//! Utilities for monad-related functions

use lambda_calculus::*;
use combinators::{I, Z};
use data::list::pair as pair_list;

/// Creates a `foldm` function specialized for a monadic type.
///
/// FOLDM :: (b -> a -> M b) -> M b -> [a] -> M b
///
/// FOLDM ≡ (λfal.Z (λzfsl.IS_NIL l (λx.s) (λx.z f (AND_THEN s (λx.f x (HEAD l))) (TAIL l)) I) f (SOME a) l
///       ≡ (λ λ λ Z (λ λ λ λ IS_NIL 1 (λ 3) (λ 5 4 (AND_THEN 3 (λ 5 1 (HEAD 3))) (TAIL 2)) I) 3 (SOME 2) 1
pub fn make_foldm(bind: Term, _return: Term) -> Term {
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
                    bind,
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
        app(_return, Var(2)),
        Var(1)
    ))
}
