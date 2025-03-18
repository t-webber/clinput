use std::fs;

use clinput::App;

fn main() {
    let mut app = App::new();
    app.action(|interface| fs::write("file", interface.line()).unwrap());
    app.log(|err| fs::write("err", err).unwrap());
    app.run();
}
