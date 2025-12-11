use crate::sprite::DesktopGremlin;
use anyhow::Result;

pub mod sprite;
pub mod ui;
pub mod utils;

fn main() -> Result<()> {
    let mut app = DesktopGremlin::new(None)?;
    app.go();
    Ok(())
}
