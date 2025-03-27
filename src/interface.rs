//! Defines the [`AppInterface`] data structure, used to interface with the
//! client.

use core::cmp;
use core::mem::take;

/// Interface provided to the client to configure the CLI.
///
/// It provides functionalities to update and manage the execution. For
/// instance, it provides ways to exit the program.
#[derive(Default)]
pub struct AppInterface<'line> {
    /// Current line to execute
    line: &'line str,
    /// Informs the runner what to do at the next step.
    status: ReturnStatus,
}

impl<'line> AppInterface<'line> {
    /// Clear the screen.
    pub fn clear_screen(&mut self) {
        self.status.add(ReturnStatus::ClearScreen);
    }

    /// Kill the process.
    pub fn exit(&mut self) {
        self.status.add(ReturnStatus::Exit);
    }

    /// Get the currently executed line.
    #[must_use]
    pub const fn line(&self) -> &str {
        self.line
    }

    /// Creates a default [`AppInterface`]
    pub(super) const fn new(line: &'line str) -> Self {
        Self { line, status: ReturnStatus::None }
    }

    /// Returns the [`ReturnStatus`]
    pub(super) fn take_status(&mut self) -> ReturnStatus {
        take(&mut self.status)
    }
}

/// Actions to be executed by the runner
#[repr(u8)]
#[derive(Default, PartialOrd, PartialEq, Eq, Ord, Clone, Copy)]
pub enum ReturnStatus {
    ClearScreen = 1,
    Exit = 2,
    #[default]
    None = 0,
}

impl ReturnStatus {
    /// Specify an additional status to be executed
    ///
    /// Only the most severe one is executed because it contains the others.
    fn add(&mut self, other: Self) {
        *self = cmp::max(*self, other);
    }
}
