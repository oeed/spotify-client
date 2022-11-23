use crate::actor::{Actor, Spotify};

pub use self::play_album::PlayAlbum;
use async_trait::async_trait;
use tokio::sync::oneshot;

mod play_album;

pub enum Commands {
  PlayAlbum(CommandResponder<PlayAlbum>),
}

pub struct CommandResponder<T: Command> {
  pub command: T,
  pub respond_to: oneshot::Sender<T::Response>,
}

impl<T: Command> CommandResponder<T> {
  pub fn new(command: T) -> (Self, oneshot::Receiver<T::Response>) {
    let (send, recv) = oneshot::channel();
    (
      CommandResponder {
        command,
        respond_to: send,
      },
      recv,
    )
  }
}

#[async_trait]
pub trait Command {
  type Response;

  async fn execute(&self, spotify: &Spotify) -> Self::Response;
}
