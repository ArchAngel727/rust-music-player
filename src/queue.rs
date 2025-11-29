use std::collections::VecDeque;

#[derive(Clone)]
pub struct Queue {
    song_queue: VecDeque<String>,
}

impl Queue {
    pub fn new() -> Queue {
        Queue {
            song_queue: VecDeque::new(),
        }
    }

    pub fn add(&mut self, string: String) {
        self.song_queue.push_back(string);
    }

    pub fn pop(&mut self) {
        self.song_queue.pop_front();
    }

    pub fn get_queue(&self) -> color_eyre::Result<Vec<String>> {
        Ok(self.song_queue.clone().into())
    }

    pub fn get_current_song(&self) -> color_eyre::Result<Option<String>> {
        if !self.song_queue.is_empty() {
            if let Some(split) = self.song_queue[0].clone().rsplit_once(".") {
                Ok(Some(split.0.to_string()))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    pub fn get_next_song(&self) -> color_eyre::Result<Option<String>> {
        if self.song_queue.len() > 1 {
            Ok(Some(self.song_queue[1].clone()))
        } else {
            Ok(None)
        }
    }

    pub fn clear(&mut self) {
        self.song_queue.clear();
    }
}
