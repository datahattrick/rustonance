use songbird::input::YoutubeDl;
use crate::utils::{Context, Error};
use poise::serenity_prelude as serenity;

// Event related imports to detect track creation failures.

use serenity::async_trait;

use crate::messaging::message::check_msg;
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
pub async fn play(ctx: Context<'_>, url: String) -> CommandResult {
    let do_search = !url.starts_with("http");

    let guild_id = ctx.guild_id().unwrap();
    let data = ctx.data();

    if let Some(handler_lock) = data.songbird.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let src = if do_search {
            YoutubeDl::new_search(data.http_client.clone(), url)
        } else {
            YoutubeDl::new(data.http_client.clone(), url)
        };
        let _ = handler.play_input(src.into());

        check_msg(ctx.say("Playing song").await);

        handler.remove_all_global_events()
    } else {
        check_msg(ctx.say("Not in a voice channel to play in").await);
    }

    Ok(())
}

