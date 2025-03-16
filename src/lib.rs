use core::mem::take;
use crossterm::{
    event::{Event, KeyCode, read},
    terminal::{disable_raw_mode, enable_raw_mode},
};

#[derive(Default)]
struct Line {
    content: String,
    cursor: usize,
}

impl Line {
    fn insert(&mut self, ch: char) {
        self.content.insert(self.cursor, ch);
    }
    fn backspace(&mut self) {
        if self.cursor > 0 {
            self.content.pop();
            self.cursor -= 1;
        }
    }
    fn decrease_counter(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }
    fn increase_counter(&mut self) {
        self.cursor += 1;
    }
    fn take(&mut self) -> String {
        take(self).content
    }
}

#[derive(Default)]
struct History {
    content: Vec<Box<str>>,
    cursor: usize,
}

impl History {
    fn push(&mut self, line: Box<str>) {
        self.content.push(line);
    }
}

pub struct App<F: Fn(&str)> {
    action: F,
    line: Line,
    history: History,
}

impl<F: Fn(&str)> From<F> for App<F> {
    fn from(action: F) -> Self {
        enable_raw_mode().expect("Terminal not supported. Please raise an issue.");

        Self {
            action,
            line: Line::default(),
            history: History::default(),
        }
    }
}

impl<F: Fn(&str)> App<F> {
    pub fn run(&mut self) {
        while self.step() {}
        disable_raw_mode().expect("Terminal not supported. Please raise an issue.");
    }

    fn take_action(&mut self) {
        let line = self.line.take();
        (self.action)(&line);
        self.history.push(line.into_boxed_str());
    }

    fn step(&mut self) -> bool {
        if let Event::Key(key) = read().unwrap() {
            match key.code {
                KeyCode::Enter => self.take_action(),
                KeyCode::Char(ch) => self.line.insert(ch),
                KeyCode::Backspace => self.line.backspace(),
                KeyCode::Esc => return false,
                KeyCode::Left => self.line.decrease_counter(),
                KeyCode::Right => self.line.increase_counter(),
                _ => (),
            }
        }
        true
    }
}
