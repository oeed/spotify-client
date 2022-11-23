use crate::actor::Spotify;

use super::Command;
use async_trait::async_trait;

pub struct PlayAlbum {
  pub album_id: String,
}

#[async_trait]
impl Command for PlayAlbum {
  type Response = u16;

  async fn execute(&self, spotify: &Spotify) -> Self::Response {
    spotify.playback.play_album(&self.album_id).await.unwrap();
    0
  }
}
