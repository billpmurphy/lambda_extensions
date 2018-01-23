extern crate lambda_extensions;

use lambda_extensions::*;
use lambda_extensions::data::char::*;

#[test]
fn test_char_convert() {
    assert_eq!('\x00'.into_church(), 0.into_church());
    assert_eq!('1'.into_church(), 49.into_church());
    assert_eq!('a'.into_church(), 97.into_church());
    assert_eq!('\x7F'.into_church(), 127.into_church());
    assert_eq!('é'.into_church(), 233.into_church());
}

#[test]
#[ignore]
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
#[ignore]
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
#[ignore]
fn test_is_ascii_alpha() {
    assert_eq!(beta(app(is_ascii_alpha(), '9'.into_church()), NOR, 0), false.into());
    assert_eq!(beta(app(is_ascii_alpha(), '@'.into_church()), NOR, 0), false.into());
    assert_eq!(beta(app(is_ascii_alpha(), 'A'.into_church()), NOR, 0), true.into());
    assert_eq!(beta(app(is_ascii_alpha(), 'G'.into_church()), NOR, 0), true.into());
    assert_eq!(beta(app(is_ascii_alpha(), 'L'.into_church()), NOR, 0), true.into());
    assert_eq!(beta(app(is_ascii_alpha(), 'O'.into_church()), NOR, 0), true.into());
    assert_eq!(beta(app(is_ascii_alpha(), 'Z'.into_church()), NOR, 0), true.into());
    assert_eq!(beta(app(is_ascii_alpha(), '['.into_church()), NOR, 0), false.into());
    assert_eq!(beta(app(is_ascii_alpha(), '`'.into_church()), NOR, 0), false.into());
    assert_eq!(beta(app(is_ascii_alpha(), 'a'.into_church()), NOR, 0), true.into());
    assert_eq!(beta(app(is_ascii_alpha(), 'g'.into_church()), NOR, 0), true.into());
    assert_eq!(beta(app(is_ascii_alpha(), 'l'.into_church()), NOR, 0), true.into());
    assert_eq!(beta(app(is_ascii_alpha(), 'o'.into_church()), NOR, 0), true.into());
    assert_eq!(beta(app(is_ascii_alpha(), 'z'.into_church()), NOR, 0), true.into());
    assert_eq!(beta(app(is_ascii_alpha(), '{'.into_church()), NOR, 0), false.into());
}

#[test]
#[ignore]
fn test_is_ascii_whitespace() {
    assert_eq!(beta(app(is_ascii_whitespace(), ' '.into_church()), NOR, 0), true.into());
    assert_eq!(beta(app(is_ascii_whitespace(), '\n'.into_church()), NOR, 0), true.into());
    assert_eq!(beta(app(is_ascii_whitespace(), '\r'.into_church()), NOR, 0), true.into());
    assert_eq!(beta(app(is_ascii_whitespace(), '\x0C'.into_church()), NOR, 0), true.into());
    assert_eq!(beta(app(is_ascii_whitespace(), '\t'.into_church()), NOR, 0), true.into());
    assert_eq!(beta(app(is_ascii_whitespace(), '['.into_church()), NOR, 0), false.into());
    assert_eq!(beta(app(is_ascii_whitespace(), '`'.into_church()), NOR, 0), false.into());
    assert_eq!(beta(app(is_ascii_whitespace(), '\x00'.into_church()), NOR, 0), false.into());
}

#[test]
#[ignore]
fn test_is_ascii_upper() {
    assert_eq!(beta(app(is_ascii_upper(), '9'.into_church()), NOR, 0), false.into());
    assert_eq!(beta(app(is_ascii_upper(), '@'.into_church()), NOR, 0), false.into());
    assert_eq!(beta(app(is_ascii_upper(), 'A'.into_church()), NOR, 0), true.into());
    assert_eq!(beta(app(is_ascii_upper(), 'G'.into_church()), NOR, 0), true.into());
    assert_eq!(beta(app(is_ascii_upper(), 'L'.into_church()), NOR, 0), true.into());
    assert_eq!(beta(app(is_ascii_upper(), 'O'.into_church()), NOR, 0), true.into());
    assert_eq!(beta(app(is_ascii_upper(), 'Z'.into_church()), NOR, 0), true.into());
    assert_eq!(beta(app(is_ascii_upper(), '['.into_church()), NOR, 0), false.into());
    assert_eq!(beta(app(is_ascii_upper(), '`'.into_church()), NOR, 0), false.into());
    assert_eq!(beta(app(is_ascii_upper(), 'a'.into_church()), NOR, 0), false.into());
    assert_eq!(beta(app(is_ascii_upper(), 'g'.into_church()), NOR, 0), false.into());
    assert_eq!(beta(app(is_ascii_upper(), 'l'.into_church()), NOR, 0), false.into());
    assert_eq!(beta(app(is_ascii_upper(), 'o'.into_church()), NOR, 0), false.into());
    assert_eq!(beta(app(is_ascii_upper(), 'z'.into_church()), NOR, 0), false.into());
    assert_eq!(beta(app(is_ascii_upper(), '{'.into_church()), NOR, 0), false.into());
}

#[test]
#[ignore]
fn test_is_ascii_lower() {
    assert_eq!(beta(app(is_ascii_lower(), '9'.into_church()), NOR, 0), false.into());
    assert_eq!(beta(app(is_ascii_lower(), '@'.into_church()), NOR, 0), false.into());
    assert_eq!(beta(app(is_ascii_lower(), 'A'.into_church()), NOR, 0), false.into());
    assert_eq!(beta(app(is_ascii_lower(), 'G'.into_church()), NOR, 0), false.into());
    assert_eq!(beta(app(is_ascii_lower(), 'L'.into_church()), NOR, 0), false.into());
    assert_eq!(beta(app(is_ascii_lower(), 'O'.into_church()), NOR, 0), false.into());
    assert_eq!(beta(app(is_ascii_lower(), 'Z'.into_church()), NOR, 0), false.into());
    assert_eq!(beta(app(is_ascii_lower(), '['.into_church()), NOR, 0), false.into());
    assert_eq!(beta(app(is_ascii_lower(), '`'.into_church()), NOR, 0), false.into());
    assert_eq!(beta(app(is_ascii_lower(), 'a'.into_church()), NOR, 0), true.into());
    assert_eq!(beta(app(is_ascii_lower(), 'g'.into_church()), NOR, 0), true.into());
    assert_eq!(beta(app(is_ascii_lower(), 'l'.into_church()), NOR, 0), true.into());
    assert_eq!(beta(app(is_ascii_lower(), 'o'.into_church()), NOR, 0), true.into());
    assert_eq!(beta(app(is_ascii_lower(), 'z'.into_church()), NOR, 0), true.into());
    assert_eq!(beta(app(is_ascii_lower(), '{'.into_church()), NOR, 0), false.into());
}

#[test]
#[ignore]
fn test_to_ascii_upper() {
    assert_eq!(beta(app(to_ascii_upper(), '9'.into_church()), NOR, 0), '9'.into_church());
    assert_eq!(beta(app(to_ascii_upper(), '@'.into_church()), NOR, 0), '@'.into_church());
    assert_eq!(beta(app(to_ascii_upper(), 'A'.into_church()), NOR, 0), 'A'.into_church());
    assert_eq!(beta(app(to_ascii_upper(), 'G'.into_church()), NOR, 0), 'G'.into_church());
    assert_eq!(beta(app(to_ascii_upper(), 'Z'.into_church()), NOR, 0), 'Z'.into_church());
    assert_eq!(beta(app(to_ascii_upper(), '['.into_church()), NOR, 0), '['.into_church());
    assert_eq!(beta(app(to_ascii_upper(), '`'.into_church()), NOR, 0), '`'.into_church());
    assert_eq!(beta(app(to_ascii_upper(), 'a'.into_church()), NOR, 0), 'A'.into_church());
    assert_eq!(beta(app(to_ascii_upper(), 'g'.into_church()), NOR, 0), 'G'.into_church());
    assert_eq!(beta(app(to_ascii_upper(), 'l'.into_church()), NOR, 0), 'L'.into_church());
    assert_eq!(beta(app(to_ascii_upper(), 'o'.into_church()), NOR, 0), 'O'.into_church());
    assert_eq!(beta(app(to_ascii_upper(), 'z'.into_church()), NOR, 0), 'Z'.into_church());
    assert_eq!(beta(app(to_ascii_upper(), '{'.into_church()), NOR, 0), '{'.into_church());
}

#[test]
#[ignore]
fn test_to_ascii_lower() {
    assert_eq!(beta(app(to_ascii_lower(), '9'.into_church()), NOR, 0), '9'.into_church());
    assert_eq!(beta(app(to_ascii_lower(), '@'.into_church()), NOR, 0), '@'.into_church());
    assert_eq!(beta(app(to_ascii_lower(), 'A'.into_church()), NOR, 0), 'a'.into_church());
    assert_eq!(beta(app(to_ascii_lower(), 'G'.into_church()), NOR, 0), 'g'.into_church());
    assert_eq!(beta(app(to_ascii_lower(), 'L'.into_church()), NOR, 0), 'l'.into_church());
    assert_eq!(beta(app(to_ascii_lower(), 'O'.into_church()), NOR, 0), 'o'.into_church());
    assert_eq!(beta(app(to_ascii_lower(), 'Z'.into_church()), NOR, 0), 'z'.into_church());
    assert_eq!(beta(app(to_ascii_lower(), '['.into_church()), NOR, 0), '['.into_church());
    assert_eq!(beta(app(to_ascii_lower(), '`'.into_church()), NOR, 0), '`'.into_church());
    assert_eq!(beta(app(to_ascii_lower(), 'a'.into_church()), NOR, 0), 'a'.into_church());
    assert_eq!(beta(app(to_ascii_lower(), 'l'.into_church()), NOR, 0), 'l'.into_church());
    assert_eq!(beta(app(to_ascii_lower(), 'z'.into_church()), NOR, 0), 'z'.into_church());
    assert_eq!(beta(app(to_ascii_lower(), '{'.into_church()), NOR, 0), '{'.into_church());
}

#[test]
#[ignore]
fn test_to_digit() {
    let none: Option<usize> = None;
    assert_eq!(beta(app(to_digit(), '0'.into_church()), NOR, 0), Some(0).into_church());
    assert_eq!(beta(app(to_digit(), '1'.into_church()), NOR, 0), Some(1).into_church());
    assert_eq!(beta(app(to_digit(), '2'.into_church()), NOR, 0), Some(2).into_church());
    assert_eq!(beta(app(to_digit(), '3'.into_church()), NOR, 0), Some(3).into_church());
    assert_eq!(beta(app(to_digit(), '4'.into_church()), NOR, 0), Some(4).into_church());
    assert_eq!(beta(app(to_digit(), '5'.into_church()), NOR, 0), Some(5).into_church());
    assert_eq!(beta(app(to_digit(), '6'.into_church()), NOR, 0), Some(6).into_church());
    assert_eq!(beta(app(to_digit(), '7'.into_church()), NOR, 0), Some(7).into_church());
    assert_eq!(beta(app(to_digit(), '8'.into_church()), NOR, 0), Some(8).into_church());
    assert_eq!(beta(app(to_digit(), '9'.into_church()), NOR, 0), Some(9).into_church());
    assert_eq!(beta(app(to_digit(), 'a'.into_church()), NOR, 0), none.into_church());
    assert_eq!(beta(app(to_digit(), 'A'.into_church()), NOR, 0), none.into_church());
    assert_eq!(beta(app(to_digit(), ':'.into_church()), NOR, 0), none.into_church());
}
