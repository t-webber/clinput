use cli_history_input::App;

fn main() {
    App::from(|line| println!(">>> {line}")).run();
}
