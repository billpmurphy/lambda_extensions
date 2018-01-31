extern crate lambda_extensions;

use lambda_extensions::*;
use lambda_extensions::data::char::*;
use lambda_extensions::utils::assert_lc;

#[test]
fn test_char_convert() {
    assert_lc('\x00'.into_church(), 0.into_church());
    assert_lc('1'.into_church(), 49.into_church());
    assert_lc('a'.into_church(), 97.into_church());
    assert_lc('\x7F'.into_church(), 127.into_church());
    assert_lc('é'.into_church(), 233.into_church());
    assert_lc('é'.into_church(), 233.into_church());
}

#[test]
#[ignore]
fn test_is_ascii() {
    assert_lc(app(is_ascii(), '\x00'.into_church()), true);
    assert_lc(app(is_ascii(), '\x01'.into_church()), true);
    assert_lc(app(is_ascii(), '1'.into_church()), true);
    assert_lc(app(is_ascii(), 'a'.into_church()), true);
    assert_lc(app(is_ascii(), '\x7F'.into_church()), true);
    assert_lc(app(is_ascii(), 'Ç'.into_church()), false);
    assert_lc(app(is_ascii(), 'é'.into_church()), false);
}

#[test]
#[ignore]
fn test_is_ascii_digit() {
    assert_lc(app(is_ascii_digit(), '/'.into_church()), false);
    assert_lc(app(is_ascii_digit(), '0'.into_church()), true);
    assert_lc(app(is_ascii_digit(), '1'.into_church()), true);
    assert_lc(app(is_ascii_digit(), '5'.into_church()), true);
    assert_lc(app(is_ascii_digit(), '9'.into_church()), true);
    assert_lc(app(is_ascii_digit(), ':'.into_church()), false);
    assert_lc(app(is_ascii_digit(), 'a'.into_church()), false);
}

#[test]
#[ignore]
fn test_is_ascii_alpha() {
    assert_lc(app(is_ascii_alpha(), '9'.into_church()), false);
    assert_lc(app(is_ascii_alpha(), '@'.into_church()), false);
    assert_lc(app(is_ascii_alpha(), 'A'.into_church()), true);
    assert_lc(app(is_ascii_alpha(), 'G'.into_church()), true);
    assert_lc(app(is_ascii_alpha(), 'L'.into_church()), true);
    assert_lc(app(is_ascii_alpha(), 'O'.into_church()), true);
    assert_lc(app(is_ascii_alpha(), 'Z'.into_church()), true);
    assert_lc(app(is_ascii_alpha(), '['.into_church()), false);
    assert_lc(app(is_ascii_alpha(), '`'.into_church()), false);
    assert_lc(app(is_ascii_alpha(), 'a'.into_church()), true);
    assert_lc(app(is_ascii_alpha(), 'g'.into_church()), true);
    assert_lc(app(is_ascii_alpha(), 'l'.into_church()), true);
    assert_lc(app(is_ascii_alpha(), 'o'.into_church()), true);
    assert_lc(app(is_ascii_alpha(), 'z'.into_church()), true);
    assert_lc(app(is_ascii_alpha(), '{'.into_church()), false);
}

#[test]
#[ignore]
fn test_is_ascii_whitespace() {
    assert_lc(app(is_ascii_whitespace(), ' '.into_church()), true);
    assert_lc(app(is_ascii_whitespace(), '\n'.into_church()), true);
    assert_lc(app(is_ascii_whitespace(), '\r'.into_church()), true);
    assert_lc(app(is_ascii_whitespace(), '\x0C'.into_church()), true);
    assert_lc(app(is_ascii_whitespace(), '\t'.into_church()), true);
    assert_lc(app(is_ascii_whitespace(), '['.into_church()), false);
    assert_lc(app(is_ascii_whitespace(), '`'.into_church()), false);
    assert_lc(app(is_ascii_whitespace(), '\x00'.into_church()), false);
}

#[test]
#[ignore]
fn test_is_ascii_upper() {
    assert_lc(app(is_ascii_upper(), '9'.into_church()), false);
    assert_lc(app(is_ascii_upper(), '@'.into_church()), false);
    assert_lc(app(is_ascii_upper(), 'A'.into_church()), true);
    assert_lc(app(is_ascii_upper(), 'G'.into_church()), true);
    assert_lc(app(is_ascii_upper(), 'L'.into_church()), true);
    assert_lc(app(is_ascii_upper(), 'O'.into_church()), true);
    assert_lc(app(is_ascii_upper(), 'Z'.into_church()), true);
    assert_lc(app(is_ascii_upper(), '['.into_church()), false);
    assert_lc(app(is_ascii_upper(), '`'.into_church()), false);
    assert_lc(app(is_ascii_upper(), 'a'.into_church()), false);
    assert_lc(app(is_ascii_upper(), 'g'.into_church()), false);
    assert_lc(app(is_ascii_upper(), 'l'.into_church()), false);
    assert_lc(app(is_ascii_upper(), 'o'.into_church()), false);
    assert_lc(app(is_ascii_upper(), 'z'.into_church()), false);
    assert_lc(app(is_ascii_upper(), '{'.into_church()), false);
}

#[test]
#[ignore]
fn test_is_ascii_lower() {
    assert_lc(app(is_ascii_lower(), '9'.into_church()), false);
    assert_lc(app(is_ascii_lower(), '@'.into_church()), false);
    assert_lc(app(is_ascii_lower(), 'A'.into_church()), false);
    assert_lc(app(is_ascii_lower(), 'G'.into_church()), false);
    assert_lc(app(is_ascii_lower(), 'L'.into_church()), false);
    assert_lc(app(is_ascii_lower(), 'O'.into_church()), false);
    assert_lc(app(is_ascii_lower(), 'Z'.into_church()), false);
    assert_lc(app(is_ascii_lower(), '['.into_church()), false);
    assert_lc(app(is_ascii_lower(), '`'.into_church()), false);
    assert_lc(app(is_ascii_lower(), 'a'.into_church()), true);
    assert_lc(app(is_ascii_lower(), 'g'.into_church()), true);
    assert_lc(app(is_ascii_lower(), 'l'.into_church()), true);
    assert_lc(app(is_ascii_lower(), 'o'.into_church()), true);
    assert_lc(app(is_ascii_lower(), 'z'.into_church()), true);
    assert_lc(app(is_ascii_lower(), '{'.into_church()), false);
}

#[test]
#[ignore]
fn test_to_ascii_upper() {
    assert_lc(app(to_ascii_upper(), '9'.into_church()), '9'.into_church());
    assert_lc(app(to_ascii_upper(), '@'.into_church()), '@'.into_church());
    assert_lc(app(to_ascii_upper(), 'A'.into_church()), 'A'.into_church());
    assert_lc(app(to_ascii_upper(), 'G'.into_church()), 'G'.into_church());
    assert_lc(app(to_ascii_upper(), 'Z'.into_church()), 'Z'.into_church());
    assert_lc(app(to_ascii_upper(), '['.into_church()), '['.into_church());
    assert_lc(app(to_ascii_upper(), '`'.into_church()), '`'.into_church());
    assert_lc(app(to_ascii_upper(), 'a'.into_church()), 'A'.into_church());
    assert_lc(app(to_ascii_upper(), 'g'.into_church()), 'G'.into_church());
    assert_lc(app(to_ascii_upper(), 'l'.into_church()), 'L'.into_church());
    assert_lc(app(to_ascii_upper(), 'o'.into_church()), 'O'.into_church());
    assert_lc(app(to_ascii_upper(), 'z'.into_church()), 'Z'.into_church());
    assert_lc(app(to_ascii_upper(), '{'.into_church()), '{'.into_church());
}

#[test]
#[ignore]
fn test_to_ascii_lower() {
    assert_lc(app(to_ascii_lower(), '9'.into_church()), '9'.into_church());
    assert_lc(app(to_ascii_lower(), '@'.into_church()), '@'.into_church());
    assert_lc(app(to_ascii_lower(), 'A'.into_church()), 'a'.into_church());
    assert_lc(app(to_ascii_lower(), 'G'.into_church()), 'g'.into_church());
    assert_lc(app(to_ascii_lower(), 'L'.into_church()), 'l'.into_church());
    assert_lc(app(to_ascii_lower(), 'O'.into_church()), 'o'.into_church());
    assert_lc(app(to_ascii_lower(), 'Z'.into_church()), 'z'.into_church());
    assert_lc(app(to_ascii_lower(), '['.into_church()), '['.into_church());
    assert_lc(app(to_ascii_lower(), '`'.into_church()), '`'.into_church());
    assert_lc(app(to_ascii_lower(), 'a'.into_church()), 'a'.into_church());
    assert_lc(app(to_ascii_lower(), 'l'.into_church()), 'l'.into_church());
    assert_lc(app(to_ascii_lower(), 'z'.into_church()), 'z'.into_church());
    assert_lc(app(to_ascii_lower(), '{'.into_church()), '{'.into_church());
}

#[test]
#[ignore]
fn test_to_digit() {
    let none: Option<usize> = None;
    assert_lc(app(to_digit(), '0'.into_church()), Some(0).into_church());
    assert_lc(app(to_digit(), '1'.into_church()), Some(1).into_church());
    assert_lc(app(to_digit(), '2'.into_church()), Some(2).into_church());
    assert_lc(app(to_digit(), '3'.into_church()), Some(3).into_church());
    assert_lc(app(to_digit(), '4'.into_church()), Some(4).into_church());
    assert_lc(app(to_digit(), '5'.into_church()), Some(5).into_church());
    assert_lc(app(to_digit(), '6'.into_church()), Some(6).into_church());
    assert_lc(app(to_digit(), '7'.into_church()), Some(7).into_church());
    assert_lc(app(to_digit(), '8'.into_church()), Some(8).into_church());
    assert_lc(app(to_digit(), '9'.into_church()), Some(9).into_church());
    assert_lc(app(to_digit(), 'a'.into_church()), none.into_church());
    assert_lc(app(to_digit(), 'A'.into_church()), none.into_church());
    assert_lc(app(to_digit(), ':'.into_church()), none.into_church());
}
