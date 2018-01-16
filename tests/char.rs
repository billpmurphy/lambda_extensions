extern crate lambda_extensions;

use lambda_extensions::*;
use lambda_extensions::data::char::*;

#[test]
fn test_is_ascii() {
    assert_eq!(beta(app(is_ascii(), '\x00'.into_church()), NOR, 0), true.into());
    assert_eq!(beta(app(is_ascii(), '\x01'.into_church()), NOR, 0), true.into());
    assert_eq!(beta(app(is_ascii(), '1'.into_church()), NOR, 0), true.into());
    assert_eq!(beta(app(is_ascii(), 'a'.into_church()), NOR, 0), true.into());
    assert_eq!(beta(app(is_ascii(), '\x7F'.into_church()), NOR, 0), true.into());
    assert_eq!(beta(app(is_ascii(), 'Ç'.into_church()), NOR, 0), false.into());
    assert_eq!(beta(app(is_ascii(), 'é'.into_church()), NOR, 0), false.into());
}

#[test]
fn test_is_ascii_digit() {
    assert_eq!(beta(app(is_ascii_digit(), 47.into_church()), NOR, 0), false.into());
    assert_eq!(beta(app(is_ascii_digit(), '0'.into_church()), NOR, 0), true.into());
    assert_eq!(beta(app(is_ascii_digit(), '1'.into_church()), NOR, 0), true.into());
    assert_eq!(beta(app(is_ascii_digit(), '5'.into_church()), NOR, 0), true.into());
    assert_eq!(beta(app(is_ascii_digit(), '9'.into_church()), NOR, 0), true.into());
    assert_eq!(beta(app(is_ascii_digit(), ':'.into_church()), NOR, 0), false.into());
    assert_eq!(beta(app(is_ascii_digit(), 'a'.into_church()), NOR, 0), false.into());
}

#[test]
fn test_is_ascii_alpha() {
    assert_eq!(beta(app(is_ascii_alpha(), '9'.into_church()), NOR, 0), false.into());
    assert_eq!(beta(app(is_ascii_alpha(), '@'.into_church()), NOR, 0), false.into());
    assert_eq!(beta(app(is_ascii_alpha(), 'A'.into_church()), NOR, 0), true.into());
    assert_eq!(beta(app(is_ascii_alpha(), 'L'.into_church()), NOR, 0), true.into());
    assert_eq!(beta(app(is_ascii_alpha(), 'Z'.into_church()), NOR, 0), true.into());
    assert_eq!(beta(app(is_ascii_alpha(), '['.into_church()), NOR, 0), false.into());
    assert_eq!(beta(app(is_ascii_alpha(), '`'.into_church()), NOR, 0), false.into());
    assert_eq!(beta(app(is_ascii_alpha(), 'a'.into_church()), NOR, 0), true.into());
    assert_eq!(beta(app(is_ascii_alpha(), 'l'.into_church()), NOR, 0), true.into());
    assert_eq!(beta(app(is_ascii_alpha(), 'z'.into_church()), NOR, 0), true.into());
    assert_eq!(beta(app(is_ascii_alpha(), '{'.into_church()), NOR, 0), false.into());
}
