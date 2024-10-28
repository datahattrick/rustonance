
use crate::sources::youtube::YouTube;
use crate::model::{Error,Context};

use crate::messaging::message::{check_msg, create_music_embed, create_music_message, send_music_message};
use crate::commands::join::join_channel;
//use crate::sources::spotify;


#[derive(Clone)]
pub enum QueryType {
    Keywords(String),
    KeywordList(Vec<String>),
    VideoLink(String),
    PlaylistLink(String),
}

#[poise::command(
    guild_only, 
    track_edits, 
    slash_command, 
    category = "Music")]
/// Will attempt to get and play the requested song
pub async fn play(ctx: Context<'_>, 
    #[autocomplete = "poise::builtins::autocomplete_command"]
    #[description = "What you would like to play, this can be a keyword(s) or a URL"] 
    message: Vec<String>
) ->  Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    join_channel(ctx).await?;
    let query = message.join(" ");

    check_msg(ctx.say("Grabbing and adding it to the queue").await);
    if let Some(handler_lock) = ctx.data().songbird.get(guild_id) {
        let mut handler = handler_lock.lock().await;
    
        let track = YouTube::query(ctx, query).await.unwrap();
        let track_info = YouTube::info(track.clone()).await;

        handler.enqueue_input(track.into()).await;
        let queue = handler.queue().current_queue();


        let embed = create_music_embed(track_info.name.clone(), track_info.artists.join(" "), track_info.duration, queue.len(), track_info.image_url.clone());
        let message = create_music_message(embed).await;
        // let context = ctx.serenity_context().clone();
        // handler.add_global_event(::songbird::Event::Track(::songbird::TrackEvent::Play), NextInQueueNotifier {
        //     title: track_info.name.clone(),
        //     artists: track_info.artists,
        //     image_url: track_info.image_url,
        //     duration: track_info.duration,
        //     queue: queue.len(),
        //     channel_id: ctx.channel_id(),
        //     ctx: context,
        // });

        if handler.queue().len() == 1 {
            send_music_message(ctx.serenity_context(), ctx.channel_id(), message).await;
        } else {
        check_msg(
            ctx.say(format!("Added {} to queue: position {}", track_info.name, handler.queue().len()),)
                .await,
            );
        };

    } else {
        check_msg(ctx.say("Not in a voice channel to play in").await);
    }

    Ok(())
}


