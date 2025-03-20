//! Runner for the application

use core::fmt;
use std::io;

use crossterm::event::{Event, KeyCode, read};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

use crate::history::History;
use crate::interface::AppInterface;
use crate::line::Line;
use crate::print_code_line_flush;

/// Log the error if it exists
///
/// The error is logged in the way specified by the user. If no way was
/// provided, nothing is done.
macro_rules! log_error {
    ($self:ident, $result:expr) => {
        match $result {
            Ok(val) => Some(val),
            Err(err) => {
                if let Some(log) = &mut $self.log {
                    log(format!("[ERROR] {err:?}"));
                }
                None
            }
        }
    };
}

impl<A> Action for A where A: FnMut(&mut AppInterface<'_>) {}

/// Type of an action
///
/// The action is what is executed on every line submission with the
/// [`KeyCode::Enter`] key.
pub trait Action: FnMut(&mut AppInterface<'_>) {}

/// Application data containing the current line and the history of executed
/// commands.
pub struct App<A: Action, L: Log> {
    /// Action executed every line
    action: Option<A>,
    /// History of submitted lines
    history: History,
    /// Current line
    line: Line,
    /// Action executed when an error occurs
    log: Option<L>,
}

impl<A: Action, L: Log> App<A, L> {
    /// Log some information in a way wanted by the user not to pollute the
    /// terminal
    fn log_info(&mut self, info: impl fmt::Debug) {
        if let Some(log) = &mut self.log {
            log(format!("[INFO] {info:?}"));
        }
    }

    /// Main runner for one line.
    fn step(&mut self) -> Result<bool, io::Error> {
        if let Some(Event::Key(key)) = log_error!(self, read()) {
            match key.code {
                KeyCode::Enter => return Ok(self.take_action()),
                KeyCode::Char(ch) => {
                    self.log_info(format!("Insert {ch}."));
                    self.line.insert(ch)?;
                }
                KeyCode::Backspace => self.line.backspace()?,
                KeyCode::Esc => return Ok(true),
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
                | KeyCode::Modifier(_) => {
                    self.log_info(format!("Pressed unsupported {:?}.", key.code));
                }
            }
        }
        Ok(false)
    }

    /// Submit the action
    fn take_action(&mut self) -> bool {
        print!("\n\r");
        let line = self.line.take();
        let mut interface = AppInterface::new(&line);
        if let Some(action) = &mut self.action {
            action(&mut interface);
        }
        if interface.get_exit() {
            return true;
        }
        log_error!(self, self.history.push(line.into_boxed_str()));
        log_error!(self, print_code_line_flush(""));
        false
    }
}

impl<A: Action, L: Log> App<A, L> {
    /// Sets the action of the app
    pub fn action(&mut self, action: A) {
        self.action = Some(action);
    }

    /// Stores the history of entered commands
    ///
    /// This allows the user to go back in history even after the program is
    /// killed. This is possible by storing the history of entered commands in a
    /// file (the same principle as the `.bash_history` file).
    pub fn history(&mut self, path: String) {
        log_error!(self, self.history.store(path));
    }

    /// Sets the logger of the app
    pub fn log(&mut self, log: L) {
        self.log = Some(log);
    }

    /// Creates an empty [`App`]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the infinite loop on the line inputs
    ///
    /// - On enter press, execute the line.
    /// - On escape press, exit the runner.
    pub fn run(&mut self) {
        self.log_info("CLI started");
        log_error!(self, self.history.load());
        log_error!(self, print_code_line_flush(""));
        loop {
            match self.step() {
                Ok(true) => break,
                res => {
                    log_error!(self, res);
                }
            }
            log_error!(self, self.line.update_cursor());
        }
        print!("\r");
        log_error!(self, disable_raw_mode());
    }
}

impl<A: Action, L: Log> Default for App<A, L> {
    fn default() -> Self {
        enable_raw_mode().unwrap_or_default();
        Self { action: None, history: History::default(), line: Line::default(), log: None }
    }
}

impl<L: FnMut(String)> Log for L {}

/// Type of a log
///
/// The log is what is executed in case of error. This allows the users to store
/// the errors somewhere without killing the program.
pub trait Log: FnMut(String) {}
