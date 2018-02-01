extern crate lambda_calculus;
extern crate lambda_extensions;

use lambda_extensions::*;
use lambda_calculus::data::boolean;

#[test]
fn test_convert_bool_from_term() {
    assert_eq!(abs(Var(1)).try_from(), None);
    assert_eq!(abs!(3, Var(0)).try_from(), None);
    assert_eq!(boolean::fls().try_from(), Some(false));
    assert_eq!(boolean::tru().try_from(), Some(true));
}
