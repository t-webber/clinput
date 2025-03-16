use core::mem::take;
use crossterm::{
    event::{Event, KeyCode, read},
    terminal::{disable_raw_mode, enable_raw_mode},
};

type LineContent = String;

pub struct App<F: Fn(LineContent)> {
    line_func: F,
    current_line: String,
    line_cursor: usize,
}

impl<F: Fn(LineContent)> From<F> for App<F> {
    fn from(line_func: F) -> Self {
        enable_raw_mode().expect("Terminal not supported. Please raise an issue.");

        Self {
            line_func,
            current_line: String::new(),
            line_cursor: 0,
        }
    }
}

impl<F: Fn(LineContent)> App<F> {
    fn insert(&mut self, ch: char) {
        self.current_line.insert(self.line_cursor, ch);
    }
    fn backspace(&mut self) {
        if self.line_cursor > 0 {
            self.current_line.pop();
            self.line_cursor -= 1;
        }
    }
    fn decrease_counter(&mut self) {
        if self.line_cursor > 0 {
            self.line_cursor -= 1;
        }
    }

    pub fn run(&mut self) {
        while self.step() {}
        disable_raw_mode().expect("Terminal not supported. Please raise an issue.");
    }

    fn step(&mut self) -> bool {
        if let Event::Key(key) = read().unwrap() {
            match key.code {
                KeyCode::Enter => (self.line_func)(take(&mut self.current_line)),
                KeyCode::Char(ch) => self.insert(ch),
                KeyCode::Backspace => self.backspace(),
                KeyCode::Esc => return false,
                KeyCode::Left => self.decrease_counter(),
                _ => (),
            }
        }
        true
    }
}
