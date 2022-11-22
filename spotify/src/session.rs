pub use librespot::core::session::Session as LibrespotSession;
use librespot::core::{authentication::Credentials, config::SessionConfig};

use crate::Playback;

pub struct Session {
  pub session: LibrespotSession,
  pub credentials: Credentials,
}

impl Session {
  pub fn new(username: &str, password: &str) -> Self {
    let session_config = SessionConfig::default();
    let credentials = Credentials::with_password(username, password);
    let session = LibrespotSession::new(session_config, None);

    Session { session, credentials }
  }

  pub async fn playback(&self) -> Playback {
    let (playback, connection) = Playback::connect(&self).await;
    connection.run_in_background();
    playback
  }
}
