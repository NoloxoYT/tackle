// All of this code was wrote by hand, please respect it by following the guideline in contributing.md
mod backend;
mod packages;
mod tui;
use anyhow::bail;
use sudo;

use crate::packages::load;
fn main() -> anyhow::Result<()> {
    if sudo::check() != sudo::RunningAs::Root {
        let elevation = sudo::escalate_if_needed();
        if elevation.is_err() {
            bail!("Failed to run as sudo")
        }
    }
    load::get();
    ratatui::run(tui::start)?;
    Ok(())
}
