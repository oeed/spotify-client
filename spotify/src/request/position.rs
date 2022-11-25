use crate::{actor::Spotify, session::Session};

use super::Request;
use async_trait::async_trait;

type Milliseconds = u32;

#[derive(Debug)]
pub struct Position;

#[async_trait]
impl Request for Position {
  type Response = Milliseconds;

  async fn request(self, spotify: &Spotify) -> Result<Self::Response, crate::Error> {
    spotify.playback.position_ms().await
  }
}

impl Session {
  pub async fn track_position(&self) -> Milliseconds {
    self.request(Position).await
  }
}
