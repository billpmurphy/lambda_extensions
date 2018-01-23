extern crate lambda_extensions;

use lambda_extensions::*;

fn into_church_pairlist(v: Vec<char>) -> Term {
    v.iter().map(|c| c.into_church()).collect::<Vec<Term>>().into_pair_list()
}

#[test]
fn test_string_convert() {
    assert_eq!("".into_church(), into_church_pairlist(vec![]));
    assert_eq!("a".into_church(), into_church_pairlist(vec!['a']));
    assert_eq!("abc".into_church(), into_church_pairlist(vec!['a', 'b', 'c']));
}
