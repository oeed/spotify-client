use librespot::core::{authentication::Credentials, config::SessionConfig};

pub struct Session {
  pub session: librespot::core::session::Session,
  pub credentials: Credentials,
}

impl Session {
  pub fn new(username: &str, password: &str) -> Self {
    let session_config = SessionConfig::default();
    let credentials = Credentials::with_password(username, password);
    let session = librespot::core::session::Session::new(session_config, None);

    Session { session, credentials }
  }
}
