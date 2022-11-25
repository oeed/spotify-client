use crate::{actor::Spotify, session::Session};

use super::Command;
use async_trait::async_trait;

#[derive(Debug)]
pub struct NextTrack;

#[async_trait]
impl Command for NextTrack {
  async fn execute(self, spotify: &Spotify) -> Result<(), crate::Error> {
    spotify.playback.next()
  }
}

impl Session {
  pub fn next_track(&self) {
    self.command(NextTrack)
  }
}
