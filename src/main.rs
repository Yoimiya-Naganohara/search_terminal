use handle::{Handle, Handler};

mod handle;
mod search_engine;
fn main() {
    run_cli_mode();
}

fn run_cli_mode() {
    let mut handler = Handle::new();
    handler.welcome();
    loop {
        handler.input();
        handler.handler();
    }
}
