use crate::{actor::Spotify, session::Session};

use super::Command;
use async_trait::async_trait;

#[derive(Debug)]
pub struct PreviousTrack;

#[async_trait]
impl Command for PreviousTrack {
  async fn execute(self, spotify: &Spotify) -> Result<(), crate::Error> {
    spotify.playback.prev()
  }
}

impl Session {
  pub fn previous_track(&self) {
    self.command(PreviousTrack)
  }
}
