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
#![allow(clippy::blanket_clippy_restriction_lints, reason = "enable all lints")]
#![allow(clippy::print_stdout, reason = "crate's goal")]

mod history;
mod line;

use core::fmt::Debug;
use std::io::{self, Write as _, stdout};

use crossterm::event::{Event, KeyCode, read};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use history::History;
use line::Line;

/// Size of the prefix displayed at every start of line.
const PREFIX_SIZE: u16 = 4;

/// Application data containing the current line and the history of executed
/// commands.
pub struct App<F: Fn(&str), L: Fn(String)> {
    /// Action executed every line
    action: F,
    /// History of submitted lines
    history: History,
    /// Current line
    line: Line,
    /// Action executed when an error occurs
    log: Option<L>,
}

impl<F: Fn(&str), L: Fn(String)> App<F, L> {
    /// Log errors in a way wanted by the user not to pollute the terminal
    fn log_error<T>(&self, result: Result<T, impl Debug>) -> Option<T> {
        match result {
            Ok(val) => Some(val),
            Err(err) => {
                if let Some(log) = &self.log {
                    log(format!("[ERROR] {err:?}"));
                }
                None
            }
        }
    }

    /// Creates an app with a given line action
    pub fn new_with_action(action: F) -> Self {
        enable_raw_mode().unwrap_or_default();
        Self { action, history: History::default(), line: Line::default(), log: None }
    }

    /// Creates an app with a given line action
    pub fn new_with_action_log(action: F, log: L) -> Self {
        let this =
            Self { action, history: History::default(), line: Line::default(), log: Some(log) };
        this.log_error(enable_raw_mode());
        this
    }

    /// Run the infinite loop on the line inputs
    ///
    /// - On enter press, execute the line.
    /// - On escape press, exit the runner.
    pub fn run(&mut self) {
        self.log_error(print_code_line_flush(""));
        loop {
            match self.step() {
                Ok(false) => break,
                res => {
                    self.log_error(res);
                }
            }
        }
        self.log_error(disable_raw_mode());
    }

    /// Main runner for one line.
    fn step(&mut self) -> Result<bool, io::Error> {
        if let Some(Event::Key(key)) = self.log_error(read()) {
            match key.code {
                KeyCode::Enter => self.take_action()?,
                KeyCode::Char(ch) => self.line.insert(ch)?,
                KeyCode::Backspace => self.line.backspace()?,
                KeyCode::Esc => return Ok(false),
                KeyCode::Left => self.line.decrease_counter(),
                KeyCode::Right => self.line.increase_counter(),
                KeyCode::Up =>
                    if let Some(line) = self.history.up() {
                        self.line.set(line.to_owned())?;
                    },
                KeyCode::Down =>
                    if let Some(line) = self.history.down() {
                        self.line.set(line.to_owned())?;
                    },
                KeyCode::Home
                | KeyCode::End
                | KeyCode::PageUp
                | KeyCode::PageDown
                | KeyCode::Tab
                | KeyCode::BackTab
                | KeyCode::Delete
                | KeyCode::Insert
                | KeyCode::F(_)
                | KeyCode::Null
                | KeyCode::CapsLock
                | KeyCode::ScrollLock
                | KeyCode::NumLock
                | KeyCode::PrintScreen
                | KeyCode::Pause
                | KeyCode::Menu
                | KeyCode::KeypadBegin
                | KeyCode::Media(_)
                | KeyCode::Modifier(_) => (),
            }
        }
        self.line.update_cursor().map(|()| true)
    }

    /// Submit the action
    ///
    /// This is called when [`KeyCode::Enter`] is pressed.
    fn take_action(&mut self) -> Result<(), io::Error> {
        println!();
        let line = self.line.take();
        (self.action)(&line);
        self.history.push(line.into_boxed_str());
        print_code_line_flush("")
    }
}

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
