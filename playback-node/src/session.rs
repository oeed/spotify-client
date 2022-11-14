use librespot::{
  core::{
    authentication::Credentials,
    config::{ConnectConfig, SessionConfig},
    session::Session,
    spotify_id::SpotifyId,
  },
  playback::{
    audio_backend,
    config::{AudioFormat, PlayerConfig},
    mixer::NoOpVolume,
    player::Player,
  },
};

pub struct SpotifySession {
  session: Session,
  player: Player,
}

impl SpotifySession {
  pub async fn new(username: String, password: String) -> Self {
    let session_config = SessionConfig::default();
    let player_config = PlayerConfig::default();
    let connect_config = ConnectConfig::default();
    let audio_format = AudioFormat::default();
    let credentials = Credentials::with_password(username, password);

    let (session, _) = Session::connect(session_config, credentials, None, false)
      .await
      .unwrap();

    let backend = audio_backend::find(None).unwrap();
    let (player, _) = Player::new(player_config, session.clone(), Box::new(NoOpVolume), move || {
      backend(None, audio_format)
    });

    SpotifySession { session, player }
  }
}
