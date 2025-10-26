mod app;
mod browser;
mod ui;

use crate::app::App;

fn main() -> color_eyre::Result<()> {
    let mut app = App::new();

    let terminal = ratatui::init();
    let _ = app.run(terminal);
    ratatui::restore();
    Ok(())
}
