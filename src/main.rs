mod app;
mod browser;
mod player;
mod player_controller;
mod player_controller_message;
mod player_message;
mod queue;
mod song;
mod ui;

use crate::app::App;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let app = App::new();

    let terminal = ratatui::init();
    app?.run(terminal)?;
    ratatui::restore();
    Ok(())
}
