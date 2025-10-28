use crate::player::PlayerCommand;
use std::path::PathBuf;

#[derive(Clone)]
pub struct PlayerMessage {
    command: PlayerCommand,
    message: Option<PathBuf>,
}

impl PlayerMessage {
    pub fn new(command: PlayerCommand, message: Option<PathBuf>) -> PlayerMessage {
        PlayerMessage { command, message }
    }

    pub fn get_command(&self) -> PlayerCommand {
        self.command.clone()
    }

    pub fn get_message(&self) -> color_eyre::Result<Option<PathBuf>> {
        Ok(self.message.clone())
    }
}
