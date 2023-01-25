use crate::{actor::Spotify, session::Session};

use super::Request;
use async_trait::async_trait;
use librespot::core::spclient::OffsetLimit;

#[derive(Debug)]
pub struct LibraryAlbums;

#[async_trait]
impl Request for LibraryAlbums {
  type Response = Vec<LibraryAlbum>;

  async fn request(self, spotify: &Spotify) -> Result<Self::Response, crate::Error> {
    // TODO: currently we don't bother with pagination as we've set the limit high enough
    spotify
      .session
      .spclient()
      .get_library_albums(OffsetLimit { offset: 0, limit: 500 })
      .await
      .map(|data| {
        data
          .items
          .into_iter()
          .map(|album| LibraryAlbum {
            uri: album.album.uri.to_uri().unwrap(),
            name: album.album.data.name,
            artists: album
              .album
              .data
              .artists
              .items
              .into_iter()
              .map(|artist| LibraryArtist {
                uri: artist.uri.to_uri().unwrap(),
                name: artist.profile.name,
              })
              .collect(),
            cover_url: album
              .album
              .data
              .cover_art
              .sources
              .into_iter()
              .max_by(|a, b| a.width.cmp(&b.width))
              .map(|art| art.url)
              .unwrap(),
            release_date: album.album.data.date.unix_timestamp(),
          })
          .collect()
      })
  }
}

impl Session {
  pub async fn library_albums(&self) -> Vec<LibraryAlbum> {
    self.request(LibraryAlbums).await
  }
}

type UnixMillisecondsTimestamp = i64;

pub struct LibraryAlbum {
  uri: String,
  name: String,
  artists: Vec<LibraryArtist>,
  cover_url: String,
  release_date: UnixMillisecondsTimestamp,
}

// TODO: swift-bridge doesn't allow non-opaque types from being in Vec, change to struct types once possible
impl LibraryAlbum {
  pub fn uri(&self) -> String {
    self.uri.clone()
  }

  pub fn name(&self) -> String {
    self.name.clone()
  }

  pub fn artists(&self) -> &Vec<LibraryArtist> {
    &self.artists
  }

  pub fn release_date(&self) -> UnixMillisecondsTimestamp {
    self.release_date
  }
}

pub struct LibraryArtist {
  uri: String,
  name: String,
}

// TODO: swift-bridge doesn't allow non-opaque types from being in Vec, change to struct types once possible
impl LibraryArtist {
  pub fn uri(&self) -> String {
    self.uri.clone()
  }

  pub fn name(&self) -> String {
    self.name.clone()
  }
}
