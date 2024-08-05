use std::str::FromStr;
use std::env;
use spotify_rs::{auth::{NoVerifier, Token}, client::Client, model::track::Track, AuthCodeClient, AuthCodeFlow, RedirectUrl};
use tokio::sync::Mutex;
use regex::Regex;
use crate::error::RustonanceError;
use crate::commands::music::QueryType;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref SPOTIFY: Mutex<Result<Client<Token, AuthCodeFlow, NoVerifier>, RustonanceError>> =
        Mutex::new(Err(RustonanceError::Other("no auth attempts")));
    pub static ref SPOTIFY_QUERY_REGEX: Regex =
        Regex::new(r"spotify.com/(?P<media_type>.+)/(?P<media_id>.*?)(?:\?|$)").unwrap();
}

#[derive(Clone, Copy)]
pub enum MediaType {
    Track,
    Album,
    Playlist,
}

impl FromStr for MediaType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "track" => Ok(Self::Track),
            "album" => Ok(Self::Album),
            "playlist" => Ok(Self::Playlist),
            _ => Err(()),
        }
    }
}


pub struct Spotify {}

impl Spotify {
    pub async fn auth() -> Result<Client<Token, AuthCodeFlow, NoVerifier>, RustonanceError> {
        let spotify_client_id = env::var("SPOTIFY_CLIENT_ID")
            .map_err(|_| RustonanceError::Other("Missing spotify client ID"))?;

        let spotify_client_secret = env::var("SPOTIFY_CLIENT_SECRET")
            .map_err(|_| RustonanceError::Other("Missing spotify client secret"))?;

        let redirect_url = RedirectUrl::new("redirect_url".to_owned())?;
        let scopes = vec!["user-library-read", "playlist-read-private"];
        let auth_code_flow = AuthCodeFlow::new(spotify_client_id, spotify_client_secret, scopes);

        let (client, url) = AuthCodeClient::new(auth_code_flow, redirect_url, true);

        let spotify = client.authenticate("auth_code", "csrf_token").await?;

        Ok(spotify)
    }

    pub async fn spotify_type(
        spotify: &Client<Token, AuthCodeFlow, NoVerifier>,
        query: &str,
    ) -> Result<QueryType, RustonanceError> {
        let captures = SPOTIFY_QUERY_REGEX
            .captures(query)
            .ok_or(RustonanceError::Other("Spotify invalid query"))?;

        let media_type = captures
            .name("media_type")
            .ok_or(RustonanceError::Other("Spotify invalid query"))?
            .as_str();

        let media_type = MediaType::from_str(media_type)
            .map_err(|_| RustonanceError::Other("Spotify invalid query"))?;

        let media_id = captures
            .name("media_id")
            .ok_or(RustonanceError::Other("Spotify invalid query"))?
            .as_str();

        match media_type {
            MediaType::Track => Self::get_track(spotify, media_id).await,
            MediaType::Album => Self::get_album(spotify, media_id).await,
            MediaType::Playlist => Self::get_playlist(spotify, media_id).await,
        }
    }

    pub async fn get_track(
        spotify: &Client<Token, AuthCodeFlow, NoVerifier>,
        media_id: &str,
    ) -> Result<Track, RustonanceError> {
        let track = spotify.track(media_id).get().await?;
        Ok(track)
    }


    pub async fn get_album(
        spotify: &Client<Token, AuthCodeFlow, NoVerifier>,
        media_id: &str,
    ) -> Result<Track, RustonanceError> {
        let track = spotify.track(media_id).get().await?;
        Ok(track)
    }


    pub async fn get_playlist(
        spotify: &Client<Token, AuthCodeFlow, NoVerifier>,
        media_id: &str,
    ) -> Result<Track, RustonanceError> {
        let track = spotify.track(media_id).get().await?;
        Ok(track)
    }

}

