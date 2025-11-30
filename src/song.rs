use audiotags::Tag;
use std::path::PathBuf;

#[derive(Clone)]
#[allow(dead_code)]
pub struct Song {
    title: Option<String>,
    artist: Option<String>,
    path: PathBuf,
}

impl Song {
    pub fn new(path: PathBuf) -> Song {
        let mut title: Option<String> = None;
        let mut artist: Option<String> = None;

        if let Some(str) = path.to_str()
            && let Some(filename) = str.split("/").last()
                && let Some(split) = filename.rsplit_once(".") {
                title = Some(split.0.to_string());
        }

        artist = Tag::new()
            .read_from_path(path.clone())
            .ok()
            .and_then(|tag| tag.artist().map(|a| a.to_owned()));

        Song { title, artist, path }
    }

    pub fn get_title(&self) -> color_eyre::Result<Option<String>> {
        Ok(self.title.clone())
    }

    pub fn get_artist(&self) -> color_eyre::Result<Option<String>> {
        Ok(self.artist.clone())
    }
}
