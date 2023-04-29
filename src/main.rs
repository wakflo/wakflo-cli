mod commands;
mod api;
mod templates;
mod utils;

use human_panic::setup_panic;
use commands::WakfloCli;

fn main() {
    setup_panic!();

    WakfloCli::init().expect("something wrong");
}
