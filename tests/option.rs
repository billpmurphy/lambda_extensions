extern crate lambda_calculus;
extern crate lambda_extensions;

use lambda_extensions::*;
use lambda_calculus::data::boolean;
use lambda_calculus::data::option;

#[test]
fn test_convert_term_to_option_bool() {
    assert_eq!(abs(Var(1)).try_from() as Option<Option<bool>>, None);
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
