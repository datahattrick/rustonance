
use crate::sources::youtube::YouTube;
use crate::utils::{Error,Context};

use crate::messaging::message::check_msg;
use crate::commands::join::join_channel;
//use crate::sources::spotify;


#[derive(Clone)]
pub enum QueryType {
    Keywords(String),
    KeywordList(Vec<String>),
    VideoLink(String),
    PlaylistLink(String),
}

#[poise::command(prefix_command, guild_only)]
pub async fn play(ctx: Context<'_>, url: String) ->  Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();

    join_channel(ctx).await?;

    if let Some(handler_lock) = ctx.data().songbird.get(guild_id) {
        let mut handler = handler_lock.lock().await;
    
        let track = YouTube::query(ctx, url.to_string()).await.unwrap();
        let _track_info = YouTube::info(track.clone()).await;

        handler.enqueue_input(track.into()).await;
        check_msg(
            ctx.say(format!("Added song to queue: position {}", handler.queue().len()),)
                .await,
            );
    } else {
        check_msg(ctx.say("Not in a voice channel to play in").await);
    }

    Ok(())
}


