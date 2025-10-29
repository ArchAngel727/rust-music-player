use rodio::{Decoder, Sink};
use std::{fs::File, io::BufReader, path::PathBuf};

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
}

impl Player {
    pub fn new() -> Player {
        Player {
            player_state: PlayerState::Paused,
            sink: None,
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
}
