//! Conversion traits for lambda-encoded options

use lambda_calculus::*;
use data::convert::*;

macro_rules! make_option_trait {
    ($trait_name:ident, $trait_fn:ident, $type_name:ident) => (
        impl $trait_name<Option<$type_name>> for Term {
            fn $trait_fn(&self) -> Option<Option<$type_name>> {
                match self.unabs_ref().and_then(|x| x.unabs_ref()) {
                    Ok(&App(ref f, ref v)) if Ok(&1) == f.unvar_ref() => v.$trait_fn().map(Some),
                    Ok(&Var(2)) => Some(None),
                    _ => None,
                }
            }
        }
    );
}

make_option_trait!(TryFromTerm, try_from, bool);

make_option_trait!(TryFromTermChurch, try_from_church, bool);
make_option_trait!(TryFromTermChurch, try_from_church, char);
make_option_trait!(TryFromTermChurch, try_from_church, u64);
