//! Manage the submitted command history

use core::mem::take;
use std::io;

use crate::{print_code_line, print_code_line_flush};

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
    pub fn backspace(&mut self) {
        #[expect(clippy::arithmetic_side_effects, reason = "manual check")]
        if self.cursor > 0 {
            self.content.pop();
            self.cursor -= 1;
        }
    }

    /// Move the cursor left
    pub fn decrease_counter(&mut self) {
        #[expect(clippy::arithmetic_side_effects, reason = "manual check")]
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    /// Move the cursor right
    pub fn increase_counter(&mut self) {
        self.cursor = self.cursor.saturating_add(1);
    }

    /// Insert a character into the line
    pub fn insert(&mut self, ch: char) -> Result<(), io::Error> {
        self.content.insert(self.cursor, ch);
        self.increase_counter();
        print_code_line_flush(&self.content)
    }

    /// Sets the whole line
    pub fn set(&mut self, line: String) -> Result<(), io::Error> {
        self.cursor = line.len().checked_sub(1).unwrap_or_default();
        print_code_line(&" ".repeat(self.content.len()));
        self.content = line;
        print_code_line_flush(&self.content)
    }

    /// Resets the line and returns the content
    pub fn take(&mut self) -> String {
        take(self).content
    }
}
