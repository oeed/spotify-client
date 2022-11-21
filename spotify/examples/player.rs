use spotify::Playback;

#[tokio::main]
async fn main() {
  let (playback, connection) = Playback::connect("1241876757", "testpassword").await;
  playback
    .play_context(String::from("spotify:album:7FkJxlcljM6Ix0pC2JSNOE"))
    .unwrap();

  connection.run().await;
}
