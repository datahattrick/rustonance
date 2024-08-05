use std::{error::Error, fmt};
use std::fmt::{Debug, Display};
use serenity::prelude::SerenityError;
use spotify_rs::Error as SpotifyRSClientError;

#[derive(Debug)]
pub enum RustonanceError {
    Other(&'static str),
    NotInRange(&'static str, isize, isize, isize),
    Serenity(SerenityError),
    Spotify(SpotifyRSClientError),
    IO(std::io::Error),
}

impl Error for RustonanceError {}

impl Display for RustonanceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Other(msg) => f.write_str(msg),
            Self::NotInRange(param, value, lower, upper) => f.write_str(&format!(
                "`{param}` should be between {lower} and {upper} but was {value}"
            )),
            Self::Serenity(err) => f.write_str(&format!("{err}")),
            Self::Spotify(err) => f.write_str(&format!("{err}")),
            Self::IO(err) => f.write_str(&format!("{err}"))
        }
    }
}

impl From<std::io::Error> for RustonanceError {
    fn from(err: std::io::Error) -> Self {
        Self::IO(err)
    }
}

impl From<SerenityError> for RustonanceError {
    fn from(err: SerenityError) -> Self {
        match err {
            SerenityError::NotInRange(param, value, lower, upper) => {
                Self::NotInRange(param, value as isize, lower as isize, upper as isize)
            }
            SerenityError::Other(msg) => Self::Other(msg),
            _ => Self::Serenity(err),
        }
    }
}

impl From<SpotifyRSClientError> for RustonanceError {
    fn from(err: SpotifyRSClientError) -> RustonanceError {
        RustonanceError::Spotify(err)
    }
}