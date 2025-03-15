use std::mem::take;

use crossterm::event::{Event, KeyCode, read};

type Line = Box<str>;

pub struct App<F: Fn(Line)> {
    line_func: F,
}

impl<F: Fn(Line)> From<F> for App<F> {
    fn from(line_func: F) -> Self {
        Self { line_func }
    }
}

impl<F: Fn(Line)> App<F> {
    pub fn run(&self) {
        let mut current_line = String::new();
        loop {
            self.step(&mut current_line)
        }
    }

    fn step(&self, current_line: &mut String) {
        if let Event::Key(key) = read().unwrap() {
            match key.code {
                KeyCode::Enter => (self.line_func)(take(current_line).into_boxed_str()),
                KeyCode::Char(ch) => current_line.push(ch),
                KeyCode::Backspace => {
                    current_line.pop();
                }
                _ => (),
            }
        }
    }
}
