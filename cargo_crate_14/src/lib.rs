//! # My Crate
//!
//! `cargo_crate_14` is a collection of stuff that makes no sense
//! but is there to satisfy my learning needs
//!

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = cargo_crate_14::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
///
/// # Errors
/// You must handle your errors which can be Foo and Bar
pub fn add_one(x: i32) -> i32 {
	x + 1
}
