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
    /// Goes down one in the history
    pub fn down(&mut self) -> Option<&str> {
        if self.cursor < self.content.len() {
            self.cursor = self.cursor.saturating_add(1);
            self.content.get(self.cursor).map(|line| &**line)
        } else {
            None
        }
    }

    /// Push a new line into the history
    pub fn push(&mut self, line: Box<str>) {
        self.content.push(line);
        self.cursor = self.content.len();
    }

    /// Goes up one in the history
    pub fn up(&mut self) -> Option<&str> {
        self.cursor = self.cursor.checked_sub(1).unwrap_or_default();
        self.content.get(self.cursor).map(|line| &**line)
    }
}
