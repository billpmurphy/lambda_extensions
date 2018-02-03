//! Conversion traits for lambda-encoded options

use lambda_calculus::*;
use data::convert::*;

macro_rules! make_option_trait {
    ($trait_name:ident, $trait_fn:ident, $type_name:ident) => (
        impl $trait_name<Option<$type_name>> for Term {
            fn $trait_fn(&self) -> Option<Option<$type_name>> {
                let inner: &Term = match self.unabs_ref().and_then(|x| x.unabs_ref()) {
                    Ok(&Var(2)) => return Some(None),
                    Ok(&App(ref f, ref v)) if Ok(&1) == f.unvar_ref() => v,
                    _ => return None,
                };
                inner.$trait_fn().map(|x| Some(x))
            }
        }
    );
}

make_option_trait!(TryFromTerm, try_from, bool);

make_option_trait!(TryFromTermChurch, try_from_church, bool);
make_option_trait!(TryFromTermChurch, try_from_church, char);
make_option_trait!(TryFromTermChurch, try_from_church, u64);
