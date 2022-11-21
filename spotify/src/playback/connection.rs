use librespot::{
  connect::{config::ConnectConfig, spirc::Spirc},
  playback::{
    audio_backend,
    config::{AudioFormat, PlayerConfig},
    mixer::{softmixer::SoftMixer, Mixer, MixerConfig, NoOpVolume},
    player::Player,
  },
};
use std::{future::Future, pin::Pin};

use crate::{session::Session, Playback};

impl<'a> Playback<'a> {
  pub async fn connect(session: &'a Session) -> (Playback<'a>, Connection<impl Future<Output = ()>>) {
    let player_config = PlayerConfig::default();
    let audio_format = AudioFormat::default();
    let connect_config = ConnectConfig::default();
    let backend = audio_backend::find(None).unwrap();

    let player = Player::new(
      player_config,
      session.session.clone(),
      Box::new(NoOpVolume),
      move || backend(None, audio_format),
    );

    let (spirc, spirc_task) = Spirc::new(
      connect_config,
      session.session.clone(),
      session.credentials.clone(),
      player,
      Box::new(SoftMixer::open(MixerConfig::default())),
    )
    .await
    .unwrap();

    (Playback { spirc, session }, Connection::new(spirc_task))
  }
}

pub struct Connection<T: Future<Output = ()> + Send + 'static>(Pin<Box<T>>);

impl<T: Future<Output = ()> + Send + 'static> Connection<T> {
  fn new(spirc_task: T) -> Self {
    Connection(Box::pin(spirc_task))
  }

  pub async fn run(mut self) -> ! {
    loop {
      tokio::select! {
          _ = &mut self.0 => {
            // TODO: retry logic
            panic!("Spirc shut down unexpectedly");
          },
      }
    }
  }

  pub fn run_in_background(self) {
    tokio::spawn(async move { self.run() });
  }
}
