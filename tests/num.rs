extern crate lambda_extensions;

use lambda_extensions::*;
use lambda_extensions::data::convert::*;

#[test]
fn test_convert_term_to_u64() {
    assert_eq!(abs(Var(1)).try_from_church() as Option<u64>, None);
    assert_eq!(abs(abs(Var(2))).try_from_church() as Option<u64>, None);
    assert_eq!(abs(abs(app(Var(1), Var(1)))).try_from_church() as Option<u64>, None);
    assert_eq!(abs(abs(app(Var(2), Var(2)))).try_from_church() as Option<u64>, None);
    assert_eq!(0.into_church().try_from_church(), Some(0));
    assert_eq!(1.into_church().try_from_church(), Some(1));
    assert_eq!(2.into_church().try_from_church(), Some(2));
    assert_eq!(30.into_church().try_from_church(), Some(30));
}
