use spotify::{Playback, Session};

#[tokio::main]
async fn main() {
  let session = Session::new("1241876757", "testpassword");
  let (playback, connection) = Playback::connect(&session).await;
  playback
    .play_album("spotify:album:79dL7FLiJFOO0EoehUHQBv")
    .await
    .unwrap();

  connection.run().await;
}
