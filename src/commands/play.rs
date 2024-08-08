
use songbird::{Call, Driver};
use songbird::input::{Input, YoutubeDl};
use songbird::tracks::{TrackHandle, TrackQueue};
use tokio::sync::MutexGuard;
use tracing::info;
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
    let _data = ctx.data();
    let driver = ctx.data().driver.clone();

    join_channel(ctx).await?;

    let mut queues = ctx.data().queues.clone();
    let queue = queues.entry(guild_id.into()).or_default();

    if let Some(handler_lock) = ctx.data().songbird.get(guild_id) {
        let mut handler = handler_lock.lock().await;
    
        let track = YouTube::query(ctx, url.to_string()).await.unwrap();
        let _track_info = YouTube::info(track.clone()).await;
    
        // let _ = add_queue(queue, driver, track).await;
    
        // let _ = queue.resume();
        handler.play(track.into());
        handler.remove_all_global_events();
        check_msg(ctx.say("Playing song").await);
    } else {
        check_msg(ctx.say("Not in a voice channel to play in").await);
    }

    Ok(())
}


pub async fn add_queue(queue: &TrackQueue, mut driver: Driver, track: YoutubeDl) -> Result<(), Error> {
    let input = Input::Lazy(Box::new(track));
    queue.add_source(input, &mut driver).await;
    Ok(())
}


