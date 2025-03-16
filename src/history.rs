//! Manage the submitted command history

/// Contains all the previous submitted commands
#[derive(Default)]
pub struct History {
    /// List of submitted commands
    content: Vec<Box<str>>,
    /// Cursor in the search of a command
    cursor: usize,
}

impl History {
    /// Push a new line into the history
    pub fn push(&mut self, line: Box<str>) {
        self.content.push(line);
        #[expect(clippy::arithmetic_side_effects, reason = "just pushed")]
        {
            self.cursor = self.content.len() - 1;
        }
    }
}
