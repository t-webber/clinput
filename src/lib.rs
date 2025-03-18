//! Library to manage terminal input history and common keybindings.
//!
//! It is useful for creating CLI tools that need to interact with history and
//! need to execute an action for every every line entered by the user.

#![warn(
    missing_docs,
    warnings,
    deprecated_safe,
    future_incompatible,
    keyword_idents,
    let_underscore,
    nonstandard_style,
    refining_impl_trait,
    rust_2018_compatibility,
    rust_2018_idioms,
    rust_2021_compatibility,
    rust_2024_compatibility,
    unused,
    clippy::all,
    clippy::pedantic,
    clippy::style,
    clippy::perf,
    clippy::complexity,
    clippy::correctness,
    clippy::restriction,
    clippy::nursery
)]
#![allow(
    clippy::single_call_fn,
    clippy::implicit_return,
    clippy::missing_trait_methods,
    clippy::question_mark_used,
    clippy::missing_inline_in_public_items,
    reason = "bad lint"
)]
#![allow(
    clippy::mod_module_files,
    clippy::module_name_repetitions,
    clippy::pub_with_shorthand,
    clippy::unseparated_literal_suffix,
    reason = "style"
)]
#![allow(
    clippy::else_if_without_else,
    clippy::pattern_type_mismatch,
    reason = "convenient"
)]
#![allow(clippy::pub_use, reason = "simpler API")]
#![allow(clippy::blanket_clippy_restriction_lints, reason = "enable all lints")]
#![allow(clippy::print_stdout, reason = "crate's goal")]

mod history;
mod line;
mod runner;

pub use runner::App;
use std::io::{self, Write as _, stdout};

/// Size of the prefix displayed at every start of line.
const PREFIX_SIZE: u16 = 4;

/// Result to handle io errors
type IoResult = Result<(), io::Error>;

/// Print without new line but flush anyway.
fn print_code_line(line: &str) {
    print!("\r>>> {line}");
}

/// Print without new line but flush anyway.
fn print_code_line_flush(line: &str) -> Result<(), io::Error> {
    print_code_line(line);
    stdout().flush()
}
