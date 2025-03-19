//! Manage the submitted command history

use core::mem::take;
use std::io::stdout;

use crossterm::ExecutableCommand as _;
use crossterm::cursor::MoveToColumn;

use crate::{IoResult, PREFIX_SIZE, print_code_line, print_code_line_flush};

/// Contains the current line status
#[derive(Default)]
pub struct Line {
    /// Line content
    content: String,
    /// Current position of the cursor on the line
    cursor: usize,
}

impl Line {
    /// Remove a character from the line
    #[expect(clippy::arithmetic_side_effects, reason = "manual check")]
    pub fn backspace(&mut self) -> IoResult {
        if self.cursor > 0 {
            self.cursor -= 1;
            self.content.remove(self.cursor);
        }
        print_code_line_flush(&format!("{} ", self.content))
    }

    /// Move the cursor left
    #[expect(clippy::arithmetic_side_effects, reason = "manual check")]
    pub const fn decrease_counter(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    /// Move the cursor right
    #[expect(clippy::arithmetic_side_effects, reason = "manual check")]
    pub const fn increase_counter(&mut self) {
        if self.cursor < self.content.len() {
            self.cursor += 1;
        }
    }

    /// Insert a character into the line
    pub fn insert(&mut self, ch: char) -> IoResult {
        self.content.insert(self.cursor, ch);
        self.cursor = self.cursor.saturating_add(1);
        print_code_line_flush(&self.content)
    }

    /// Sets the whole line
    pub fn set(&mut self, line: String) -> IoResult {
        print_code_line(&" ".repeat(self.content.len()));
        self.cursor = line.len().checked_sub(1).unwrap_or_default();
        self.content = line;
        print_code_line_flush(&self.content)
    }

    /// Resets the line and returns the content
    pub fn take(&mut self) -> String {
        take(self).content
    }

    /// Decrease the cursor on the terminal and inside the app
    ///
    /// # Panics
    ///
    /// This panics if we can't decrement.
    #[expect(
        clippy::cast_possible_truncation,
        clippy::as_conversions,
        reason = "//TODO"
    )]
    pub fn update_cursor(&self) -> IoResult {
        stdout()
            .execute(MoveToColumn(PREFIX_SIZE.saturating_add(self.cursor as u16)))
            .map(|_| ())
    }
}
