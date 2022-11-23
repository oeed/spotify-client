use crate::{actor::Spotify, session::Session};

use super::Command;
use async_trait::async_trait;

#[derive(Debug)]
pub struct PlayAlbum {
  pub album_id: String,
}

#[async_trait]
impl Command for PlayAlbum {
  async fn execute(self, spotify: &Spotify) {
    spotify.playback.play_album(&self.album_id).await.unwrap();
  }
}

impl Session {
  pub fn play_album(&self, album_id: String) {
    self.command(PlayAlbum { album_id })
  }
}
