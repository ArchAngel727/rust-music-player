use std::{sync::mpsc, thread, sync::mpsc::TryRecvError};

use color_eyre::eyre::eyre;

use crate::{
     player::{Player, PlayerCommand, PlayerState}, player_controller_message::{ControllerCommand, PlayerControllerCommand}, player_message::PlayerMessage, queue::Queue
};

pub struct PlayerController {
    sender: mpsc::Sender<PlayerMessage>,
    receiver: Option<mpsc::Receiver<PlayerControllerCommand>>,
    player_state: PlayerState,
    pub queue: Queue
}

impl PlayerController {
    pub fn new() -> color_eyre::Result<PlayerController> {
        let (tx, rx) = mpsc::channel::<PlayerMessage>();

        let mut pc = PlayerController {
            sender: tx,
            receiver: None,
            player_state: PlayerState::Paused,
            queue: Queue::new()
        };

        pc.init_player(rx)?;

        Ok(pc)
    }

    fn init_player(&mut self, rx: mpsc::Receiver<PlayerMessage>) -> color_eyre::Result<()> {
        let (tx, receiver) = mpsc::channel();

        self.receiver = Some(receiver);

        thread::spawn(move || -> color_eyre::Result<()> {
            let mut player = Player::new(tx);

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

                            player.add_callback()?;
                        }
                    }
                    PlayerCommand::PlayPause => player.play_pause()?,
                    PlayerCommand::Skip => player.skip(),
                    PlayerCommand::Stop => player.stop(),
                }

                player.tx.send(PlayerControllerCommand::new(ControllerCommand::UpdateState, Some(player.get_player_state()?)))?;
            }
        });

        Ok(())
    }

    pub fn send_command(&mut self, command: PlayerMessage) -> color_eyre::Result<()> {
        self.sender.send(command.clone())?;

        match command.get_command() {
            PlayerCommand::Play => {
                if let Some(message) = command.get_message()?
                   && let Some(title) = message.into_os_string()
                        .into_string()
                        .expect("String conversion")
                        .split("/")
                        .last() {
                            self.queue.add(title.to_string());
                }
            },
            PlayerCommand::Stop => self.queue.clear(),
            _ => {}
        }

        //self.check_for_message()?;

        Ok(())
    }

    pub fn check_for_message(&mut self) -> color_eyre::Result<()> {
        if let Some(rx) = &self.receiver {
            let pcc = match rx.try_recv() {
                Ok(message) => message,
                Err(TryRecvError::Empty) | Err(TryRecvError::Disconnected) => return Ok(()),
            };

            match pcc.get_command() {
                ControllerCommand::UpdateState => {
                    if let Some(PlayerState::Playing) = pcc.get_state()? {
                        self.player_state = PlayerState::Playing;

                    } else {
                        self.player_state = PlayerState::Paused;
                    }
                },
                ControllerCommand::PopQueue => {
                    self.queue.pop();
                },
            }
        } else {
            return Err(eyre!("Channel does not exist!"));
        }

        Ok(())
    }

    pub fn get_player_state_as_string(&self) -> color_eyre::Result<String> {
        Ok(match self.player_state {
            PlayerState::Playing => String::from("Playing"),
            PlayerState::Paused => String::from("Paused"),
        })
    }

    pub fn toggle(&mut self) -> color_eyre::Result<()> {
        self.send_command(PlayerMessage::new(PlayerCommand::PlayPause, None))?;

        Ok(())
    }

    pub fn skip(&mut self) -> color_eyre::Result<()> {
        self.send_command(PlayerMessage::new(PlayerCommand::Skip, None))?;

        Ok(())
    }

    pub fn stop(&mut self) -> color_eyre::Result<()>{
        self.send_command(PlayerMessage::new(PlayerCommand::Stop, None))?;

        Ok(())
    }
}
