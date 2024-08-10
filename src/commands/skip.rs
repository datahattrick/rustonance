
use crate::{messaging::message::check_msg, utils::Error};

use crate::utils::Context;


#[poise::command(
    slash_command, 
    guild_only, 
    category = "Music")]
/// Will go to the next song in the queue and skip the current one.
pub async fn next(ctx: Context<'_>) -> Result<(), Error> {
    next_job(ctx).await
}

#[poise::command(
    slash_command, 
    guild_only, 
    category = "Music")]
/// Will skip the current song to the next in the queue.
pub async fn skip(ctx: Context<'_>) -> Result<(), Error> {
    next_job(ctx).await
}

pub async fn next_job(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();

    if let Some(handler_lock) = ctx.data().songbird.get(guild_id) {
        let handler = handler_lock.lock().await;

        let playing_song = handler.queue().current();
        if playing_song.is_some() {
            let _song = playing_song.unwrap();

            let queue = handler.queue();
            let _ = queue.skip();

            check_msg(ctx.say("Skipping song!").await);
        } else {
            check_msg(
                ctx.say("No Song to skip").await,
            );
        }

    } else {
        check_msg(
            ctx.say("Not in a voice channel to play in").await,
        );
    }

    Ok(())
}