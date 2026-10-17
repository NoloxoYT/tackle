// All of this code was wrote by hand, please respect it by following the guideline in contributing.md
mod backend;
mod packages;
mod tui;
use anyhow::bail;
use sudo;
fn main() -> anyhow::Result<()> {
    if sudo::check() != sudo::RunningAs::Root {
        let elevation = sudo::escalate_if_needed();
        if elevation.is_err() {
            bail!("Failed to run as sudo")
        }
    }
    ratatui::run(tui::start)?;
    Ok(())
}
