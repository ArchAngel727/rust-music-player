use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::DefaultTerminal;
use std::time::Duration;

use crate::{
    browser::{self, Browser},
    player_controller::PlayerController,
    ui,
};

pub struct App {
    running: bool,
    ui: ui::Ui,
    pub browser: browser::Browser,
    pub player_controller: PlayerController,
}

impl App {
    pub fn new() -> color_eyre::Result<App> {
        Ok(App {
            running: true,
            ui: ui::Ui::new(),
            browser: Browser::new(),
            player_controller: PlayerController::new()?,
        })
    }

    pub fn run(&mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        // init browser;
        self.browser.go_to("Music")?; // Debug code

        while self.running {
            self.player_controller.check_for_message()?;
            terminal.draw(|frame| self.ui.draw(self, frame).expect("UI Error"))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn handle_events(&mut self) -> color_eyre::Result<()> {
        if event::poll(Duration::from_millis(50))? {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event)?
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('1') => self.ui.set_current_window(ui::Window::Home),
            KeyCode::Char('2') => self.ui.set_current_window(ui::Window::Browser),
            KeyCode::Char('3') => self.ui.set_current_window(ui::Window::Queue),
            KeyCode::Char('p') => self.player_controller.toggle()?,
            KeyCode::Char('s') => self.player_controller.skip()?,
            KeyCode::Char('S') => self.player_controller.stop()?,
            KeyCode::Char('o') => println!("{:?}", self.player_controller.queue.get_queue()),
            _ => {
                if self.ui.get_current_window() == ui::Window::Browser {
                    self.browser
                        .handle_key_event(key_event, &mut self.player_controller)?
                }
            }
        }

        Ok(())
    }

    fn exit(&mut self) {
        self.running = false;
    }
}
