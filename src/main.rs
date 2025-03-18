use std::fs::{File, OpenOptions};
use std::io::Write as _;

use clinput::App;

fn appender(path: &str) -> File {
    OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)
        .unwrap()
}

fn main() {
    let mut lines = appender("lines.txt");
    let mut errors = appender("errors.txt");

    let mut app = App::new();
    app.action(|interface| writeln!(lines, "{}", interface.line()).unwrap());
    app.log(|err| writeln!(errors, "{err}").unwrap());
    app.run();
}
