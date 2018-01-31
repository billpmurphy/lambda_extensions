//! **lambda_extensions** is a collection of non-standard datatypes in the untyped lambda calculus.

#![deny(missing_docs)]
#![deny(unsafe_code)]

#[macro_use]
extern crate lambda_calculus;

// For convenience, re-export everything from `lambda_calculus`
pub use lambda_calculus::*;

pub mod utils;
pub mod data;
pub use data::char::IntoChurchChar;
pub use data::string::IntoChurchString;
