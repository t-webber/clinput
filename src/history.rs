//! Manage the submitted command history

use std::fs::{File, OpenOptions};
use std::io::{Read as _, Write as _};

use crate::IoResult;

/// Contains all the previous submitted commands
#[derive(Default)]
pub struct History {
    /// List of submitted commands
    content: Vec<Box<str>>,
    /// Cursor in the search of a command
    cursor: usize,
    /// Function to store the history on the disk
    store: Option<File>,
}

impl History {
    /// Move the cursor down
    const fn cursor_down(&mut self) {
        self.cursor = self.cursor.saturating_add(1);
    }

    /// Reset the cursor to the bottom of the pile
    const fn cursor_reset(&mut self) {
        self.cursor = self.content.len();
    }

    /// Move the cursor up
    const fn cursor_up(&mut self) {
        self.cursor = self.cursor.saturating_sub(1);
    }

    /// Goes down one in the history
    pub fn down(&mut self) -> Option<&str> {
        if self.cursor < self.content.len() {
            self.cursor_down();
            self.content.get(self.cursor).map(|line| &**line)
        } else {
            None
        }
    }

    /// Load history from the disk
    #[expect(clippy::verbose_file_reads, reason = "also used for append")]
    pub fn load(&mut self) -> IoResult {
        if let Some(store) = &mut self.store {
            let mut ancient_history = String::new();
            store.read_to_string(&mut ancient_history)?;
            self.content = ancient_history.lines().map(Into::into).collect();
            self.cursor_reset();
        }
        Ok(())
    }

    /// Push a new line into the history
    pub fn push(&mut self, line: Box<str>) -> IoResult {
        if let Some(store) = &mut self.store {
            writeln!(store, "{line}")?;
        }
        self.content.push(line);
        self.cursor_reset();
        Ok(())
    }

    /// Enables storage on the disk and provides the path to the storage file
    pub fn store(&mut self, path: String) -> IoResult {
        self.store = Some(
            OpenOptions::new()
                .append(true)
                .create(true)
                .read(true)
                .open(path)?,
        );
        Ok(())
    }

    /// Goes up one in the history
    pub fn up(&mut self) -> Option<&str> {
        self.cursor_up();
        self.content.get(self.cursor).map(|line| &**line)
    }
}
