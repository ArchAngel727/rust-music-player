use audiotags::Tag;
use color_eyre::eyre::eyre;
use crossterm::event::{KeyCode, KeyEvent};
use std::{env::home_dir, fs::read_dir, path::PathBuf};

use crate::{
    player::PlayerCommand, player_controller::PlayerController, player_message::PlayerMessage,
};

pub struct Browser {
    current_path: PathBuf,
    selected: u32,
}

impl Browser {
    pub fn new() -> Browser {
        let home: PathBuf;

        if let Some(home_dir) = home_dir() {
            home = home_dir
        } else {
            home = PathBuf::from("/home");
        }

        Browser {
            current_path: home,
            selected: 0,
        }
    }

    pub fn handle_key_event(
        &mut self,
        key_event: KeyEvent,
        pc: &mut PlayerController,
    ) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Char('h') => self.select_first()?,
            KeyCode::Char('j') => self.select_next()?,
            KeyCode::Char('k') => self.select_previous()?,
            KeyCode::Char('l') => self.select_last()?,
            KeyCode::Enter => self.select(pc)?,
            KeyCode::Backspace => self.go_back()?,
            _ => {}
        }

        Ok(())
    }

    pub fn list_dir(&self) -> color_eyre::Result<Vec<String>> {
        let mut data = read_dir(self.current_path.clone())?
            .filter_map(|item| {
                item.ok()
                    .map(|entry| {
                        entry
                            .path()
                            .into_os_string()
                            .into_string()
                            .expect("String conversion")
                    })
                    .filter(|entry| !entry.contains("/."))
            })
            .collect::<Vec<String>>();

        data.sort();

        if data
            .iter()
            .any(|entry| entry.ends_with(".mp3") || entry.ends_with(".flac"))
        {
            match self.sort_songs(&data) {
                Ok(sorted_songs) => return Ok(sorted_songs),
                Err(_e) => return Ok(data),
            };
        }

        Ok(data)
    }

    fn get_track_numbers(&self, data: &[String]) -> color_eyre::Result<Vec<u16>> {
        let track_numbers: color_eyre::Result<Vec<u16>> = data
            .iter()
            .filter(|entry| entry.ends_with(".mp3") || entry.ends_with(".flac"))
            .map(|entry| {
                Tag::new()
                    .read_from_path(entry)
                    .map_err(|e| eyre!("Failed to read tag. {e}"))?
                    .track_number()
                    .ok_or_else(|| eyre!("File does not contain track number."))
            })
            .collect();

        track_numbers
    }

    fn sort_songs(&self, data: &[String]) -> color_eyre::Result<Vec<String>> {
        let songs: Vec<String> = data
            .iter()
            .filter(|entry| entry.ends_with(".mp3") || entry.ends_with(".flac"))
            .filter_map(|entry| entry.split("/").last())
            .map(|entry| entry.to_string())
            .collect();

        let track_numbers = self.get_track_numbers(data)?;

        let mut vec_tupel: Vec<(String, u16)> = vec![];

        for i in 0..track_numbers.len() {
            vec_tupel.push((songs[i].clone(), track_numbers[i]));
        }

        vec_tupel.sort_by_key(|k| k.1);

        Ok(vec_tupel
            .into_iter()
            .map(|entry| entry.0)
            .collect::<Vec<String>>())
    }

    pub fn go_to(&mut self, path: &str) -> color_eyre::Result<()> {
        for item in self.list_dir()? {
            if item.ends_with(path) {
                self.current_path.push(path);
            }
        }

        Ok(())
    }

    fn go_back(&mut self) -> color_eyre::Result<()> {
        self.select_first()?;
        self.current_path.pop();
        Ok(())
    }

    fn select_first(&mut self) -> color_eyre::Result<()> {
        self.selected = 0;
        Ok(())
    }

    fn select_next(&mut self) -> color_eyre::Result<()> {
        if self.selected < self.get_selected_len()? - 1 {
            self.selected += 1;
        }

        Ok(())
    }

    fn select_previous(&mut self) -> color_eyre::Result<()> {
        if self.selected > 0 {
            self.selected -= 1;
        }

        Ok(())
    }

    fn select_last(&mut self) -> color_eyre::Result<()> {
        self.selected = self.get_selected_len()? - 1;
        Ok(())
    }

    pub fn select(&mut self, player_controller: &mut PlayerController) -> color_eyre::Result<()> {
        let mut path = self.current_path.clone();
        path.push(self.get_selected_path()?);

        if path.is_dir() {
            self.select_first()?;
            self.current_path = path;
        } else if path
            .as_path()
            .extension()
            .is_some_and(|ext| ext == "mp3" || ext == "flac")
        {
            player_controller.send_command(PlayerMessage::new(PlayerCommand::Play, Some(path)))?;
        }

        Ok(())
    }

    pub fn get_selected(&self) -> color_eyre::Result<u32> {
        Ok(self.selected)
    }

    fn get_selected_path(&self) -> color_eyre::Result<PathBuf> {
        Ok(PathBuf::from(
            self.list_dir()?[self.get_selected()? as usize].clone(),
        ))
    }

    fn get_selected_len(&self) -> color_eyre::Result<u32> {
        Ok(self.list_dir()?.len() as u32)
    }
}
