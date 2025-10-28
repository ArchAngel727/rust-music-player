use std::{sync::mpsc, thread};

use color_eyre::eyre::eyre;

use crate::{
    player::{Player, PlayerCommand, PlayerState},
    player_message::PlayerMessage,
};

pub struct PlayerController {
    sender: mpsc::Sender<PlayerMessage>,
    receiver: Option<mpsc::Receiver<PlayerState>>,
    player_state: PlayerState,
    current_song: String,
}

impl PlayerController {
    pub fn new() -> color_eyre::Result<PlayerController> {
        let (tx, rx) = mpsc::channel::<PlayerMessage>();

        let mut pc = PlayerController {
            sender: tx,
            receiver: None,
            player_state: PlayerState::Paused,
            current_song: String::new(),
        };

        pc.init_player(rx)?;

        Ok(pc)
    }

    fn init_player(&mut self, rx: mpsc::Receiver<PlayerMessage>) -> color_eyre::Result<()> {
        let (tx, receiver) = mpsc::channel();

        self.receiver = Some(receiver);

        thread::spawn(move || -> color_eyre::Result<()> {
            let mut player = Player::new();

            let stream_handle = rodio::OutputStreamBuilder::open_default_stream()
                .expect("open default audio stream");

            let sink = rodio::Sink::connect_new(stream_handle.mixer());

            player.set_sink(sink);

            loop {
                let command = rx.recv()?;

                match command.get_command() {
                    PlayerCommand::Play => {
                        if let Some(message) = command.get_message()? {
                            player.add_to_queue(message.clone())?;

                            player.set_player_state(PlayerState::Playing)?;
                        }
                    }
                    PlayerCommand::PlayPause => player.play_pause()?,
                    PlayerCommand::Skip => player.skip(),
                    PlayerCommand::Stop => break,
                }

                tx.send(player.get_player_state()?)?;
            }

            Ok(())
        });

        Ok(())
    }

    pub fn send_command(&mut self, message: PlayerMessage) -> color_eyre::Result<()> {
        self.sender.send(message.clone())?;

        if let Some(rx) = &self.receiver {
            if let PlayerState::Playing = rx.recv()? {
                self.player_state = PlayerState::Playing;

                if let Some(message) = message.get_message()? 
                    && let Some(title) = message
                        .into_os_string()
                        .into_string()
                        .expect("String conversion")
                        .split("/")
                        .last() {
                    self.set_current_song(title.to_string());
                }
            } else {
                self.player_state = PlayerState::Paused;
            }
        } else {
            return Err(eyre!("Catastrophe"));
        }

        Ok(())
    }

    pub fn toggle(&mut self) -> color_eyre::Result<()> {
        self.send_command(PlayerMessage::new(PlayerCommand::PlayPause, None))?;

        Ok(())
    }

    pub fn skip(&mut self) -> color_eyre::Result<()> {
        self.send_command(PlayerMessage::new(PlayerCommand::Skip, None))?;

        Ok(())
    }

    pub fn get_player_state_as_string(&self) -> color_eyre::Result<String> {
        Ok(match self.player_state {
            PlayerState::Playing => String::from("Playing"),
            PlayerState::Paused => String::from("Paused"),
        })
    }

    pub fn get_current_song(&self) -> color_eyre::Result<String> {
        Ok(self.current_song.clone())
    }

    pub fn set_current_song(&mut self, song: String) {
        self.current_song = song;
    }
}
