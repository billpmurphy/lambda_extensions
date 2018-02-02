extern crate lambda_calculus;
extern crate lambda_extensions;

use lambda_extensions::*;
use lambda_calculus::data::boolean;

#[test]
fn test_convert_term_to_bool() {
    assert_eq!(abs(Var(1)).try_from() as Option<bool>, None);
    assert_eq!(abs!(3, Var(0)).try_from() as Option<bool>, None);
    assert_eq!(boolean::fls().try_from() as Option<bool>, Some(false));
    assert_eq!(boolean::tru().try_from() as Option<bool>, Some(true));
}
