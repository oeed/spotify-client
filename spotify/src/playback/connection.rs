use librespot::{
  connect::{config::ConnectConfig, spirc::Spirc},
  core::config::DeviceType,
  playback::{
    audio_backend,
    config::{AudioFormat, PlayerConfig},
    mixer::{softmixer::SoftMixer, Mixer, MixerConfig, NoOpVolume},
    player::Player,
  },
};
use std::future::Future;

use crate::{session::Session, Playback};

impl Playback {
  pub async fn connect(session: &Session) -> (Playback, impl Future<Output = ()>) {
    let player_config = PlayerConfig::default();
    let audio_format = AudioFormat::default();
    let connect_config = ConnectConfig {
      name: String::from("Albumify"),
      device_type: DeviceType::Computer,
      initial_volume: Some(50),
      has_volume_ctrl: false, // TODO: volume control
    };
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

    (
      Playback {
        spirc,
        session: session.session.clone(),
      },
      spirc_task,
    )
  }
}

impl Drop for Playback {
  fn drop(&mut self) {
    self.spirc.disconnect().ok();
  }
}
