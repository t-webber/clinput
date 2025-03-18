use std::fs;

use clinput::App;

fn main() {
    let mut app = App::new();
    app.action(|line| fs::write("file", line).unwrap());
    app.log(|err| fs::write("err", err).unwrap());
    app.run();
}
