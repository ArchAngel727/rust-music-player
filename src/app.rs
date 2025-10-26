use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::DefaultTerminal;

use crate::{
    browser::{self, Browser},
    ui,
};

pub struct App {
    running: bool,
    ui: ui::Ui,
    pub browser: browser::Browser,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> App {
        App {
            running: true,
            ui: ui::Ui::new(),
            browser: Browser::new(),
        }
    }

    pub fn run(&mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        let _ = self.browser.go_to("Music"); // Debug code
        let _ = self.browser.go_to("Albums"); // Debug code

        while self.running {
            terminal.draw(|frame| self.ui.draw(self, frame).expect("UI Error"))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> color_eyre::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)?
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('1') => self.ui.set_current_window(ui::Window::Home),
            KeyCode::Char('2') => self.ui.set_current_window(ui::Window::Browser),
            KeyCode::Char('3') => self.ui.set_current_window(ui::Window::Queue),
            KeyCode::Char('h') => self.browser.handle_key_event(key_event)?,
            KeyCode::Char('j') => self.browser.handle_key_event(key_event)?,
            KeyCode::Char('k') => self.browser.handle_key_event(key_event)?,
            KeyCode::Char('l') => self.browser.handle_key_event(key_event)?,
            KeyCode::Enter => self.browser.handle_key_event(key_event)?,
            KeyCode::Backspace => self.browser.handle_key_event(key_event)?,
            _ => {}
        }

        Ok(())
    }

    fn exit(&mut self) {
        self.running = false;
    }
}
