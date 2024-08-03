
use std::sync::Arc;

use songbird::{input::YoutubeDl, Songbird};
use crate::utils::{Context, Error};
use poise::serenity_prelude as serenity;

// Event related imports to detect track creation failures.
use songbird::events::{Event, EventContext, EventHandler as VoiceEventHandler, TrackEvent};
use serenity::async_trait;

async fn join(ctx: Context<'_>, manager: Arc<Songbird> ) -> Result<(), Error> {
    let (guild_id, channel_id) = {
        let guild = ctx.guild().unwrap();
        let channel_id = guild
            .voice_states
            .get(&ctx.author().id)
            .and_then(|voice_state| voice_state.channel_id);

        (guild.id, channel_id)
    };

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            ctx.say("Not in a voice channel").await?;

            return Ok(());
        },
    };

    if let Ok(handler_lock) = manager.join(guild_id, connect_to).await {
        let mut handler = handler_lock.lock().await;
        handler.add_global_event(TrackEvent::Error.into(), TrackErrorNotifier)
    };

    Ok(())

}

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn play(ctx: Context<'_>,
    #[description = "Play a Song"] args: String
) -> Result<(), Error> {
    let do_search = !args.starts_with("http");
    let http_client = ctx.data().http_client();
    let guild_id = ctx.guild_id().unwrap();

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in initialisation.")
        .clone();

    join(ctx, manager.clone()).await?;

    // Retrieve the voice handler
    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let src = if do_search {
            YoutubeDl::new_search(http_client, args)
        } else {
            YoutubeDl::new(http_client, args)
        };
        ctx.say("Playing").await?;
        handler.play_input(src.clone().into());
    } else {
        ctx.say("You need to be in a Voice Channel to use this command").await?;
    }
    
    Ok(())

}


struct TrackErrorNotifier;

#[async_trait]
impl VoiceEventHandler for TrackErrorNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(track_list) = ctx {
            for (state, handle) in *track_list {
                println!(
                    "Track {:?} encountered an error: {:?}",
                    handle.uuid(),
                    state.playing
                );
            }
        }

        None
    }
}