use std::fs;

use cli_history_input::App;

fn main() {
    App::new_with_action_log(
        |line| fs::write("file", line).unwrap(),
        |err| fs::write("err", err).unwrap(),
    )
    .run();
}
