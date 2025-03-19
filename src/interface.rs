//! Defines the [`AppInterface`] data structure, used to interface with the
//! client.

/// Interface provided to the client to configure the CLI.
///
/// It provides functionalities to update and manage the execution. For
/// instance, it provides ways to exit the program.
#[derive(Default)]
pub struct AppInterface<'line> {
    /// Informs the runner to kill the runner at the next step.
    exit: bool,
    /// Current line to execute
    line: &'line str,
}

impl<'line> AppInterface<'line> {
    /// Informs the runner to kill the runner at the next step.
    pub const fn exit(&mut self) {
        self.exit = true;
    }

    /// Checks if the program needs to exit
    #[must_use]
    pub(super) const fn get_exit(&self) -> bool {
        self.exit
    }

    /// Get the currently executed line
    #[must_use]
    pub const fn line(&self) -> &str {
        self.line
    }

    /// Crates a new [`AppInterface`] for a new line
    #[must_use]
    pub(super) const fn new(line: &'line str) -> Self {
        Self { exit: false, line }
    }
}
