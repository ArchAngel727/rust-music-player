use crate::app::App;
use ratatui::{
    layout::{Direction, Flex, Layout, Rect},
    prelude::{Constraint, Stylize},
    style::{palette::tailwind, Style},
    text::Line,
    widgets::{Block, Borders, List, ListState, Paragraph},
    Frame,
};

#[derive(Clone, PartialEq)]
pub enum Window {
    Home,
    Browser,
    Queue,
}

pub struct Ui {
    current_window: Window,
}

impl Ui {
    pub fn new() -> Ui {
        Ui {
            current_window: Window::Home,
        }
    }

    pub fn get_current_window(&self) -> Window {
        self.current_window.clone()
    }

    pub fn set_current_window(&mut self, window: Window) {
        self.current_window = window;
    }

    pub fn draw(&self, app: &App, frame: &mut Frame) -> color_eyre::Result<()> {
        let layout = Layout::new(
            Direction::Vertical,
            vec![Constraint::Max(1), Constraint::Fill(1), Constraint::Max(1)],
        )
        .flex(Flex::Center)
        .split(frame.area());

        self.draw_header(frame, layout[0]);
        self.draw_footer(frame, layout[2], app)?;

        match self.get_current_window() {
            Window::Home => self.home(frame, layout[1]),
            Window::Browser => self.browser(app, frame, layout[1])?,
            Window::Queue => self.queue(app, frame, layout[1])?,
        };

        Ok(())
    }

    fn draw_header(&self, frame: &mut Frame, area: Rect) {
        let block = Block::new()
            .title(
                Line::from(format!(
                    " {} ",
                    match self.current_window {
                        Window::Home => "Home",
                        Window::Browser => "Browser",
                        Window::Queue => "Queue",
                    }
                ))
                .centered(),
            )
            .borders(Borders::TOP);

        frame.render_widget(block, area);
    }

    fn draw_footer(&self, frame: &mut Frame, area: Rect, app: &App) -> color_eyre::Result<()> {
        let block = Block::new()
            .title(
                Line::from(
                    // check lenght
                    if let Some(song) = app.player_controller.queue.get_current_song()? {
                        if let Some(title) = song.get_title()? {
                            if let Some(artist) = song.get_artist()? {
                                format!(
                                    " {}: {} - {} ",
                                    app.player_controller.get_player_state_as_string()?,
                                    artist,
                                    title
                                )
                            } else {
                                format!(
                                    " {} {} ",
                                    app.player_controller.get_player_state_as_string()?,
                                    title
                                )
                            }
                        } else {
                            format!(" {} ", app.player_controller.get_player_state_as_string()?)
                        }
                    } else {
                        format!(" {} ", app.player_controller.get_player_state_as_string()?)
                    },
                )
                .centered(),
            )
            .borders(Borders::TOP);

        frame.render_widget(block, area);

        Ok(())
    }

    fn home(&self, frame: &mut Frame, area: Rect) {
        let sub_layout = Layout::new(
            Direction::Vertical,
            vec![Constraint::Max(1), Constraint::Percentage(50)],
        )
        .flex(Flex::Center)
        .split(area);

        let main_text = Line::from("tui-music-player").bold().centered();

        frame.render_widget(main_text, sub_layout[0]);

        let paragraph = Paragraph::new("1: Home 2: Browser 3: Queue").centered();

        frame.render_widget(paragraph, sub_layout[1]);
    }

    fn browser(&self, app: &App, frame: &mut Frame, area: Rect) -> color_eyre::Result<()> {
        let mut list_state = ListState::default();

        const SELECTED_STYLE: Style = Style::new().bg(tailwind::BLUE.c700);

        let list_items: Vec<String> = app
            .browser
            .list_dir()?
            .iter()
            .filter_map(|entry| {
                let parts: Vec<&str> = entry.split("/").collect();
                if parts.len() >= 2 {
                    Some(format!(
                        "{}/{}",
                        parts[parts.len() - 2],
                        parts[parts.len() - 1]
                    ))
                } else if parts.len() == 1 {
                    Some(parts[0].to_string())
                } else {
                    None
                }
            })
            .collect();

        let list = List::new(list_items)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol("> ")
            .highlight_spacing(ratatui::widgets::HighlightSpacing::Always);

        list_state.select(Some(app.browser.get_selected()? as usize));

        frame.render_stateful_widget(list, area, &mut list_state);

        Ok(())
    }

    fn queue(&self, app: &App, frame: &mut Frame, area: Rect) -> color_eyre::Result<()> {
        let queue = app.player_controller.queue.get_queue()?;

        let mut list_items: Vec<String> = Vec::new();

        if queue.is_empty() {
            list_items.push(String::from("Empty"))
        } else {
            list_items = queue
                .iter()
                .filter_map(|song| song.get_title().ok()?)
                .enumerate()
                .map(|(i, song)| format!("{}. {:?}", i + 1, song))
                .collect();
        }

        let list = List::new(list_items);

        frame.render_widget(list, area);

        Ok(())
    }
}
