use crate::{actor::Spotify, session::Session};

use super::Command;
use async_trait::async_trait;

#[derive(Debug)]
pub struct Play;

#[async_trait]
impl Command for Play {
  async fn execute(self, spotify: &Spotify) -> Result<(), crate::Error> {
    spotify.playback.play()
  }
}

impl Session {
  pub fn play(&self) {
    self.command(Play)
  }
}
