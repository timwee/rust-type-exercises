pub mod array;
pub mod expr;
mod macros;
pub mod scalar;

use thiserror::Error;

#[derive(Error, Debug)]
#[error("Type mismatch on conversion: expected {0}, get {1}")]
pub struct TypeMismatch(&'static str, &'static str);
