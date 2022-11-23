use simple_logger::SimpleLogger;
use tokio::{
  runtime::Runtime,
  sync::mpsc::{self, UnboundedSender},
  task::JoinHandle,
};

use crate::{
  actor::Actor,
  command::{Command, Commands},
  request::{Request, RequestResponder, Requests},
};

pub struct Session {
  request_sender: UnboundedSender<Requests>,
  command_sender: UnboundedSender<Commands>,
  /// The session must own the tokio async runtime so that async code may execute in the background
  /// as swift-bridge's async runtime does not appear to always be running, or at least when we need it.
  _runtime: Runtime,
  /// As with the runtime, we need to hold on to the handle that's running the background tasks
  _runner: JoinHandle<()>,
}

impl Session {
  pub fn new(username: String, password: String) -> Session {
    SimpleLogger::new()
      .with_level(log::Level::Info.to_level_filter())
      .with_module_level("spotify_ffi", log::Level::Debug.to_level_filter())
      .init()
      .unwrap();

    // we have to host our own runtime beacuse the async runtime must last the entire duration of the app
    let runtime = Runtime::new().unwrap();
    let (request_sender, request_reciever) = mpsc::unbounded_channel();
    let (command_sender, command_reciever) = mpsc::unbounded_channel();
    Session {
      request_sender,
      command_sender,
      _runner: runtime.spawn(async move {
        Actor::new(&username, &password, request_reciever, command_reciever)
          .await
          .run()
          .await
      }),
      _runtime: runtime,
    }
  }

  pub(crate) fn command<T: Command>(&self, command: T)
  where
    T: Into<Commands>,
  {
    self
      .command_sender
      .send(command.into())
      .expect("Actor task has been killed");
  }

  pub(crate) async fn request<T: Request>(&self, request: T) -> T::Response
  where
    RequestResponder<T>: Into<Requests>,
  {
    let (responder, response) = RequestResponder::new(request);
    // we can ignore the send error because the response will have the same error
    let _ = self.request_sender.send(responder.into());
    response.await.expect("Actor task has been killed")
  }
}
