extern crate lambda_extensions;

use lambda_extensions::*;
use lambda_extensions::utils::assert_lc;
use lambda_extensions::data::list::pair::*;

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
