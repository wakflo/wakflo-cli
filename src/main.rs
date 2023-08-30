mod api;
mod commands;
mod templates;
mod utils;

use commands::WakfloCli;
use human_panic::setup_panic;

#[tokio::main]
pub async fn main() -> std::io::Result<()> {
    setup_panic!();

    WakfloCli::init().expect("something went wrong");
    Ok(())
}
