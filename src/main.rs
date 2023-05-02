mod api;
mod commands;
mod templates;
mod utils;

use commands::WakfloCli;
use human_panic::setup_panic;

fn main() {
    setup_panic!();

    WakfloCli::init().expect("something wrong");
}
