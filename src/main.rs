use std::fs;

use cli_history_input::App;

fn main() {
    App::from(|line| fs::write("file", line).unwrap()).run();
}
