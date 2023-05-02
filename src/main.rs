<<<<<<< HEAD
mod api;
mod commands;
mod templates;
mod utils;

use commands::WakfloCli;
use human_panic::setup_panic;
=======
mod commands;
mod api;
mod templates;
mod utils;

use human_panic::setup_panic;
use commands::WakfloCli;
>>>>>>> 85173fa (feat: first commit)

fn main() {
    setup_panic!();

    WakfloCli::init().expect("something wrong");
}
