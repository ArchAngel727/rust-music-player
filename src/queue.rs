use crate::song::Song;
use std::collections::VecDeque;

#[derive(Clone)]
pub struct Queue {
    song_queue: VecDeque<Song>,
}

impl Queue {
    pub fn new() -> Queue {
        Queue {
            song_queue: VecDeque::new(),
        }
    }

    pub fn add(&mut self, song: Song) {
        self.song_queue.push_back(song);
    }

    pub fn pop(&mut self) {
        self.song_queue.pop_front();
    }

    pub fn get_queue(&self) -> color_eyre::Result<Vec<Song>> {
        Ok(self.song_queue.clone().into())
    }

    pub fn get_current_song(&self) -> color_eyre::Result<Option<Song>> {
        if !self.song_queue.is_empty() {
            Ok(Some(self.song_queue[0].clone()))
        } else {
            Ok(None)
        }
    }

    pub fn clear(&mut self) {
        self.song_queue.clear();
    }
}
