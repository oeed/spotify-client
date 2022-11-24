use crate::{actor::Spotify, session::Session};

use super::Command;
use async_trait::async_trait;

#[derive(Debug)]
pub struct Pause;

#[async_trait]
impl Command for Pause {
  async fn execute(self, spotify: &Spotify) -> Result<(), crate::Error> {
    spotify.playback.pause().unwrap();
    Ok(())
  }
}

impl Session {
  pub fn pause(&self) {
    self.command(Pause)
  }
}
