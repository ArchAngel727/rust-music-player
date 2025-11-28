use rodio::{source::EmptyCallback, Decoder, Sink};
use std::{fs::File, io::BufReader, path::PathBuf, sync::mpsc};

use crate::player_controller_message::{ControllerCommand, PlayerControllerCommand};

#[derive(Clone)]
pub enum PlayerState {
    Playing,
    Paused,
}

#[derive(Clone)]
pub enum PlayerCommand {
    Play,
    PlayPause,
    Skip,
    Stop,
}

pub struct Player {
    player_state: PlayerState,
    sink: Option<Sink>,
    pub tx: mpsc::Sender<PlayerControllerCommand>,
}

impl Player {
    pub fn new(tx: mpsc::Sender<PlayerControllerCommand>) -> Player {
        Player {
            player_state: PlayerState::Paused,
            sink: None,
            tx,
        }
    }

    pub fn get_player_state(&self) -> color_eyre::Result<PlayerState> {
        Ok(self.player_state.clone())
    }

    pub fn set_player_state(&mut self, state: PlayerState) -> color_eyre::Result<()> {
        self.player_state = state;
        Ok(())
    }

    pub fn set_sink(&mut self, sink: Sink) {
        self.sink = Some(sink);
    }

    pub fn play_pause(&mut self) -> color_eyre::Result<()> {
        if let Some(sink) = &self.sink {
            if sink.is_paused() {
                sink.play();
                self.set_player_state(PlayerState::Playing)?;
            } else {
                sink.pause();
                self.set_player_state(PlayerState::Paused)?;
            }
        }

        Ok(())
    }

    pub fn add_to_queue(&self, song: PathBuf) -> color_eyre::Result<()> {
        let source = Decoder::new(BufReader::new(File::open(song)?))?;

        if let Some(sink) = &self.sink {
            sink.append(source);
            sink.play();
        }

        Ok(())
    }

    pub fn skip(&self) {
        if let Some(sink) = &self.sink {
            sink.skip_one();
        }
    }

    pub fn stop(&self) {
        if let Some(sink) = &self.sink {
            sink.stop();
        }
    }

    pub fn add_callback(&self) -> color_eyre::Result<()> {
        let tx = self.tx.clone();

        let callback = EmptyCallback::new(Box::new(move || {
            let _ = tx.send(PlayerControllerCommand::new(
                ControllerCommand::PopQueue,
                None,
            ));
        }));

        if let Some(sink) = &self.sink {
            sink.append(callback);
        }

        Ok(())
    }
}
