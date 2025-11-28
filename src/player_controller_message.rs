use crate::player::PlayerState;

#[derive(Clone)]
pub enum ControllerCommand {
    UpdateState,
    PopQueue,
}

pub struct PlayerControllerCommand {
    command: ControllerCommand,
    state: Option<PlayerState>,
}

impl PlayerControllerCommand {
    pub fn new(command: ControllerCommand, state: Option<PlayerState>) -> PlayerControllerCommand {
        PlayerControllerCommand { command, state }
    }

    pub fn get_command(&self) -> ControllerCommand {
        self.command.clone()
    }

    pub fn get_state(&self) -> color_eyre::Result<Option<PlayerState>> {
        Ok(self.state.clone())
    }
}
