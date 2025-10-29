mod app;
mod browser;
mod player;
mod player_controller;
mod player_message;
mod queue;
mod ui;

use crate::app::App;

fn main() -> color_eyre::Result<()> {
    let app = App::new();

    let terminal = ratatui::init();
    app?.run(terminal)?;
    ratatui::restore();
    Ok(())
}
