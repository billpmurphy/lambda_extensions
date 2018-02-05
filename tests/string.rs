extern crate lambda_extensions;

use lambda_extensions::*;
use lambda_extensions::data::string::*;
use lambda_extensions::utils::assert_lc;

fn into_church_pairlist(v: Vec<char>) -> Term {
    v.iter().map(|c| c.into_church()).collect::<Vec<Term>>().into_pair_list()
}

#[test]
fn test_string_convert() {
    assert_eq!("".into_church(), into_church_pairlist(vec![]));
    assert_eq!("a".into_church(), into_church_pairlist(vec!['a']));
    assert_eq!("abc".into_church(), into_church_pairlist(vec!['a', 'b', 'c']));
}

#[test]
#[ignore]
fn test_parse_num() {
    let none: Option<usize> = None;
    assert_lc(app(parse_num(), "".into_church()), none.into_church());
    assert_lc(app(parse_num(), "a".into_church()), none.into_church());
    assert_lc(app(parse_num(), "1a".into_church()), none.into_church());
    assert_lc(app(parse_num(), "a1".into_church()), none.into_church());
    assert_lc(app(parse_num(), "1a1".into_church()), none.into_church());
    assert_lc(app(parse_num(), "1 1".into_church()), none.into_church());
    assert_lc(app(parse_num(), "1".into_church()), Some(1).into_church());
    assert_lc(app(parse_num(), "01".into_church()), Some(1).into_church());
    assert_lc(app(parse_num(), "0001".into_church()), Some(1).into_church());
    assert_lc(app(parse_num(), "0010".into_church()), Some(10).into_church());
    assert_lc(app(parse_num(), "101".into_church()), Some(101).into_church());
    assert_lc(app(parse_num(), "200".into_church()), Some(200).into_church());
    assert_lc(app(parse_num(), "234".into_church()), Some(234).into_church());
}
