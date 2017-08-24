//! Chmodrt is a simple utility that recursively changes the mode of
//! directories or files and is only compatible with Unix systems. Numeric
//! modes must be an octal between one and four digits. Symbolic modes are
//! not supported.
extern crate getopts;
#[macro_use]
pub mod macros;
pub mod core;
pub mod opts;
pub mod status;
pub mod text;
