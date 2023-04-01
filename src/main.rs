mod app;
mod repertoire;

use std::io::Result;

fn main() -> Result<()> {
    app::menu();
    Ok(())
}