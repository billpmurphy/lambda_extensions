extern crate lambda_extensions;

use lambda_extensions::*;
use lambda_extensions::utils::assert_lc;
use lambda_extensions::data::list::pair::*;

#[test]
fn test_concat() {
    let empty = vec![].into_pair_list();
    let outer_empty = vec![empty.clone(), empty.clone(), empty.clone()].into_pair_list();
    let inner_single = vec![false.into()].into_pair_list();
    let inner_double = vec![false.into(), false.into()].into_pair_list();
    let outer_single = vec![inner_single.clone()].into_pair_list();
    let outer_double = vec![inner_single.clone(), inner_single.clone()].into_pair_list();
    let outer_double_2 = vec![inner_single.clone(), empty.clone(), inner_single.clone()].into_pair_list();
    let outer_double_3 = vec![empty.clone(), inner_single.clone(), inner_single.clone()].into_pair_list();

    assert_lc(app(concat(), empty.clone()), empty.clone());
    assert_lc(app(concat(), outer_empty.clone()), empty.clone());
    assert_lc(app(concat(), outer_single.clone()), inner_single.clone());
    assert_lc(app(concat(), outer_double.clone()), inner_double.clone());
    assert_lc(app(concat(), outer_double_2.clone()), inner_double.clone());
    assert_lc(app(concat(), outer_double_3.clone()), inner_double.clone());
}

#[test]
fn test_slice() {
    let empty = || vec![].into_pair_list();
    let list1 = || vec![1.into_church(), 2.into_church(), 3.into_church()].into_pair_list();
    let list2 = || vec![2.into_church(), 3.into_church()].into_pair_list();
    let list3 = || vec![1.into_church(), 2.into_church()].into_pair_list();
    let list4 = || vec![1.into_church()].into_pair_list();
    let list5 = || vec![2.into_church()].into_pair_list();

    assert_lc(app!(slice(), 0.into_church(), 0.into_church(), list1()), empty());
    assert_lc(app!(slice(), 1.into_church(), 1.into_church(), list1()), empty());
    assert_lc(app!(slice(), 2.into_church(), 2.into_church(), list1()), empty());
    assert_lc(app!(slice(), 2.into_church(), 1.into_church(), list1()), empty());
    assert_lc(app!(slice(), 2.into_church(), 0.into_church(), list1()), empty());
    assert_lc(app!(slice(), 5.into_church(), 5.into_church(), list1()), empty());
    assert_lc(app!(slice(), 0.into_church(), 5.into_church(), empty()), empty());
    assert_lc(app!(slice(), 0.into_church(), 5.into_church(), list1()), list1());
    assert_lc(app!(slice(), 1.into_church(), 5.into_church(), list1()), list2());
    assert_lc(app!(slice(), 1.into_church(), 3.into_church(), list1()), list2());
    assert_lc(app!(slice(), 0.into_church(), 2.into_church(), list1()), list3());
    assert_lc(app!(slice(), 0.into_church(), 1.into_church(), list1()), list4());
    assert_lc(app!(slice(), 1.into_church(), 2.into_church(), list1()), list5());
}

#[test]
fn test_bin_to_cnum() {
    let empty = vec![].into_pair_list();
    let list0a = vec![false.into()].into_pair_list();
    let list0b = vec![false.into(), false.into()].into_pair_list();
    let list1 = vec![true.into()].into_pair_list();
    let list2 = vec![true.into(), false.into()].into_pair_list();
    let list4 = vec![true.into(), false.into(), false.into()].into_pair_list();
    let list7a = vec![true.into(), true.into(), true.into()].into_pair_list();
    let list7b = vec![false.into(), true.into(), true.into(), true.into()].into_pair_list();
    let list10 = vec![true.into(), false.into(), true.into(), false.into()].into_pair_list();

    assert_lc(app(bin_to_cnum(), empty), 0.into_church());
    assert_lc(app(bin_to_cnum(), list0a), 0.into_church());
    assert_lc(app(bin_to_cnum(), list0b), 0.into_church());
    assert_lc(app(bin_to_cnum(), list1), 1.into_church());
    assert_lc(app(bin_to_cnum(), list2), 2.into_church());
    assert_lc(app(bin_to_cnum(), list4), 4.into_church());
    assert_lc(app(bin_to_cnum(), list7a), 7.into_church());
    assert_lc(app(bin_to_cnum(), list7b), 7.into_church());
    assert_lc(app(bin_to_cnum(), list10), 10.into_church());
}
