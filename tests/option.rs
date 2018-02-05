extern crate lambda_calculus;
extern crate lambda_extensions;

use lambda_extensions::*;
use lambda_extensions::data::option::*;
use lambda_extensions::utils::assert_lc;
use lambda_calculus::data::boolean;
use lambda_calculus::data::option;
use lambda_calculus::data::num::church;

#[test]
fn test_convert_term_to_option_bool() {
    assert_eq!(abs(Var(1)).try_from() as Option<Option<bool>>, None);
    assert_eq!(boolean::fls().try_from() as Option<Option<bool>>, None);
    assert_eq!(option::none().try_from() as Option<Option<bool>>, Some(None));
    assert_eq!(
        beta(app(option::some(), boolean::fls()), NOR, 0).try_from() as Option<Option<bool>>,
        Some(Some(false))
    );
    assert_eq!(
        beta(app(option::some(), boolean::tru()), NOR, 0).try_from() as Option<Option<bool>>,
        Some(Some(true))
    );
    assert_eq!(
        beta(app(option::some(), abs(Var(1))), NOR, 0).try_from() as Option<Option<bool>>,
        None
    );
}

#[test]
fn test_foldm_option() {
    let empty = vec![].into_pair_list();
    let list = vec![1.into_church(), 2.into_church(), 3.into_church()].into_pair_list();
    let f = || abs!(2, app(option::some(), app!(church::add(), Var(1), Var(2))));

    assert_lc(app!(foldm(), f(), 0.into_church(), empty), Some(0).into_church());
    assert_lc(app!(foldm(), f(), 0.into_church(), list.clone()), Some(6).into_church());
    assert_lc(app!(foldm(), f(), 3.into_church(), list.clone()), Some(9).into_church());
}

