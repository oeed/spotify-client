use crate::{actor::Spotify, session::Session};

use super::Request;
use async_trait::async_trait;

#[derive(Debug)]
pub struct GetState;

#[async_trait]
impl Request for GetState {
  type Response = u16;

  async fn request(self, _spotify: &Spotify) -> Result<Self::Response, crate::Error> {
    Ok(0)
  }
}

impl Session {
  pub async fn get_state(&self) -> u16 {
    self.request(GetState).await
  }
}
