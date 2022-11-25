use self::position::Position;
use crate::actor::Spotify;
use async_trait::async_trait;
use enum_dispatch::enum_dispatch;
use std::fmt::Debug;
use tokio::sync::oneshot;

mod position;

#[enum_dispatch]
pub enum Requests {
  Position(RequestResponder<Position>),
}

#[async_trait]
#[enum_dispatch(Requests)]
pub trait RequestResponderWrap {
  async fn request_send(self, spotify: &Spotify);
}

#[async_trait]
impl<T: Request> RequestResponderWrap for RequestResponder<T> {
  async fn request_send(self, spotify: &Spotify) {
    log::debug!("Request: {:?}", &self.command);
    let response = self.command.request(&spotify).await.expect("Request failed");
    // we can ignore any errors while sending: https://ryhl.io/blog/actors-with-tokio/
    let _ = self.respond_to.send(response);
  }
}

pub struct RequestResponder<T: Request> {
  pub command: T,
  pub respond_to: oneshot::Sender<T::Response>,
}

impl<T: Request> RequestResponder<T> {
  pub fn new(command: T) -> (Self, oneshot::Receiver<T::Response>) {
    let (send, recv) = oneshot::channel();
    (
      RequestResponder {
        command,
        respond_to: send,
      },
      recv,
    )
  }
}

/// Request are 'request and response' tasks the actor executes when either the response and/or waiting for it to complete is important.
/// As such, they must be called (in Swift) asynchronously.
#[async_trait]
pub trait Request: Send + Debug {
  type Response: Send;

  async fn request(self, spotify: &Spotify) -> Result<Self::Response, crate::Error>;
}
