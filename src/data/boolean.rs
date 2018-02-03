//! Lambda-encoded booleans

use lambda_calculus::*;
use data::convert::*;

impl TryFromTerm<bool> for Term {
    fn try_from(&self) -> Option<bool> {
        match self.unabs_ref().and_then(|x| x.unabs_ref()) {
            Ok(&Var(1)) => Some(false),
            Ok(&Var(2)) => Some(true),
            _ => None,
        }
    }
}

impl TryFromTermChurch<bool> for Term {
    fn try_from_church(&self) -> Option<bool> {
        self.try_from()
    }
}
