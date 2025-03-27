use std::fs::{File, OpenOptions};
use std::io::Write as _;

use clinput::{App, Key};

fn appender(path: &str) -> File {
    OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)
        .unwrap()
}

fn main() {
    let mut errors = appender("errors.txt");

    let mut app = App::new();
    app.on_submit(|interface| {
        writeln!(&mut appender("lines.txt"), "{}", interface.line()).unwrap()
    });
    app.on(Key::Escape, Box::new(|app| app.exit()));
    app.log(|err| writeln!(errors, "{err}").unwrap());
    app.history("history.txt".to_owned());
    app.run();
}
