//! **lambda_extensions** is a collection of non-standard datatypes in the untyped lambda calculus.

#![deny(missing_docs)]

#[macro_use]
extern crate lambda_calculus;

// For convenience, re-export everything from `lambda_calculus`
pub use lambda_calculus::*;

pub mod data;
pub use data::char::IntoChurchChar;
pub use data::string::IntoChurchString;
