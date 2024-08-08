use std::str::FromStr;
use std::env;
use rspotify::{
    clients::BaseClient,
    model::{SimplifiedArtist, TrackId},
    ClientCredsSpotify, Credentials, ClientError
};
use tokio::sync::Mutex;
use regex::Regex;
use tracing::{error, info};
use crate::commands::play::QueryType;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref SPOTIFY: Mutex<Result<ClientCredsSpotify, ClientError>> =
        Mutex::new(Err(ClientError::InvalidToken));

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
    pub async fn auth() -> Result<ClientCredsSpotify, ClientError> {
        let spotify_client_id = env::var("SPOTIFY_CLIENT_ID")
            .map_err(|_| error!("Missing Spotify Client ID")).unwrap();

        let spotify_client_secret = env::var("SPOTIFY_CLIENT_SECRET")
            .map_err(|_| error!("Missing Spotify Client Secret")).unwrap();

        let creds = Credentials::new(&spotify_client_id, &spotify_client_secret);

        let spotify = ClientCredsSpotify::new(creds);
        spotify.request_token().await?;
    
        Ok(spotify)
    }

    pub async fn spotify_search_type(
        spotify: ClientCredsSpotify,
        query: &str,
    ) -> QueryType {
        let captures = SPOTIFY_QUERY_REGEX
            .captures(query)
            .ok_or(info!("Spotify invalid query")).unwrap();

        let media_type = captures
            .name("media_type")
            .ok_or(info!("Spotify invalid query")).unwrap()
            .as_str();

        let media_type = MediaType::from_str(media_type)
            .map_err(|_| info!("Spotify invalid query")).unwrap();

        let media_id = captures
            .name("media_id")
            .ok_or(info!("Spotify invalid query")).unwrap()
            .as_str();

        match media_type {
            MediaType::Track => Self::get_track(spotify, media_id).await,
            MediaType::Album => Self::get_album(spotify, media_id).await,
            MediaType::Playlist => Self::get_playlist(spotify, media_id).await,
        }
    }

    pub async fn get_track(
        spotify: ClientCredsSpotify,
        media_id: &str,
    ) -> QueryType {
        info!("Getting Spotify Track {media_id}");
        let track_id = TrackId::from_id(media_id)
            .map_err(|err| error!("Track ID contains invalid characters: {err}")).unwrap();

        let track = spotify.track(track_id, None)
            .await
            .map_err(|err| info!("Track ID contains invalid characters: {:?}", err)).unwrap();

        let artist = Self::join_artists(track.artists);

        let res = Self::build_query(&artist, &track.name.clone());

        QueryType::Keywords(res)
    }

    // TODO: Get Album information
    pub async fn get_album(
        spotify: ClientCredsSpotify,
        media_id: &str,
    ) -> QueryType {
        info!("Getting Spotify Track {media_id}");
        let track_id = TrackId::from_id(media_id)
            .map_err(|err| error!("Track ID contains invalid characters: {err}")).unwrap();

        let track = spotify.track(track_id, None)
            .await
            .map_err(|err| info!("Track ID contains invalid characters: {:?}", err)).unwrap();

        let artist = Self::join_artists(track.artists);

        let res = Self::build_query(&artist, &track.name.clone());

        QueryType::Keywords(res)
    }

    // TODO: Get Playlist information
    pub async fn get_playlist(
        spotify: ClientCredsSpotify,
        media_id: &str,
    ) -> QueryType {
        info!("Getting Spotify Track {media_id}");
        let track_id = TrackId::from_id(media_id)
            .map_err(|err| error!("Track ID contains invalid characters: {err}")).unwrap();

        let track = spotify.track(track_id, None)
            .await
            .map_err(|err| info!("Track ID contains invalid characters: {:?}", err)).unwrap();

        let artist = Self::join_artists(track.artists);

        let res = Self::build_query(&artist, &track.name.clone());

        QueryType::Keywords(res)
    }

    fn build_query(artists: &str, track_name: &str) -> String {
        format!("{} - {}", artists, track_name)
    }

    fn join_artists(artists: Vec<SimplifiedArtist>) -> String {
        let artists_names: Vec<String> = artists.iter().map(
            |artist| artist.name.clone()
        )
        .collect();
        artists_names.join(" ")
    }

}
