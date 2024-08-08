use reqwest::Url;
use songbird::input::YoutubeDl;
use tracing::error;
use crate::sources::spotify::{Spotify,SPOTIFY};
use crate::utils::{Context, Error};
use poise::serenity_prelude as serenity;

// Event related imports to detect track creation failures.

use serenity::async_trait;

use crate::messaging::message::check_msg;
use crate::commands::join::join;
//use crate::sources::spotify;

type CommandResult = Result<(), Error>;

#[derive(Clone)]
pub enum QueryType {
    Keywords(String),
    KeywordList(Vec<String>),
    VideoLink(String),
    PlaylistLink(String),
}

#[poise::command(prefix_command, guild_only)]
pub async fn play(ctx: Context<'_>, url: &str) -> CommandResult {
    let guild_id = ctx.guild_id().unwrap();
    let data = ctx.data();

    // Join Voice channel if not in one yet
    join();

    let call =  data.songbird.get(guild_id).unwrap();

    // What are we playing???
    let query_type = match Url::parse(url) {
        Ok(url_data) => match url_data.host_str() {
            Some("open.spotify.com") => {
                let spotify = SPOTIFY.lock().await;
                Some(Spotify::spotify_search_type(spotify.as_ref().unwrap().clone(), url))
            },
            Some(other) => None,
            None => None,
        }
        Err(_) => None
    };
    let handler = call.lock().await;

    Ok(())
}

