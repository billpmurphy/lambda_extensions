//! Church-encoded numerals

use lambda_calculus::*;
use std::u64::MAX;

use data::convert::TryFromTermChurch;

pub use lambda_calculus::data::num::church::*;

impl TryFromTermChurch<u64> for Term {
    fn try_from_church(&self) -> Option<u64> {
        let mut inner: &Term = match self.unabs_ref().and_then(|x| x.unabs_ref()) {
            Ok(v) => v,
            _ => return None
        };

        let mut num = 0;
        while num < MAX {
            match inner {
                &Var(1) => return Some(num),
                &App(ref f, ref a) if Ok(&2) == f.unvar_ref() => {
                    num += 1;
                    inner = a;
                }
                _ => break,
            }
        }
        None
    }
}
