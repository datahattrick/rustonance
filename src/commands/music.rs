use songbird::{id::ChannelId, input::YoutubeDl};
use crate::utils::{Context, Error};
use poise::serenity_prelude as serenity;

// Event related imports to detect track creation failures.
use songbird::events::{Event, EventContext, EventHandler as VoiceEventHandler, TrackEvent};
use serenity::async_trait;

async fn join(ctx: Context<'_> ) -> Result<(), Error> {
    let (guild_id, channel_id) = {
        let guild = match ctx.guild() {
            Some(guild) => guild,
            None => {
                return Ok(());
            }
        };
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

    let manager = songbird::get(ctx.serenity_context()).await.unwrap();

    if let Some(call) = manager.get(guild_id) {
        let handler = call.lock().await;
        let in_current_call = handler.current_connection().is_some();

        if in_current_call {
            ctx.say("On my way Boss!").await?;
            let _bot_channel_id: ChannelId = handler.current_channel().unwrap().0.into();
        }
    }

    match manager.join(guild_id, connect_to).await {
        Ok(handler_lock) => {
            let mut handler = handler_lock.lock().await;
            handler.add_global_event(TrackEvent::Error.into(), TrackErrorNotifier);
        },
        Err(e) => {
            ctx.say(format!("Failed to join the channel: {}", e)).await?;
        }
    }
    Ok(())
}

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "Play a Song"] args: String
) -> Result<(), Error> {
    // Check if the input is a URL or a search term
    let do_search = !args.starts_with("http");

    // Retrieve the HTTP client from the context
    let http_client = ctx.data().http_client();

     // Ensure the command is being used in a guild
     let guild_id = match ctx.guild_id() {
        Some(id) => id,
        None => {
            ctx.say("This command can only be used in a server.").await?;
            return Ok(());
        }
    };

    // Retrieve the Songbird manager
    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in initialisation.")
        .clone();

    // Ensure the user is in a voice channel and join it
    if let Err(err) = join(ctx).await {
        ctx.say(format!("Failed to join voice channel: {}", err)).await?;
        return Ok(());
    }

    // Retrieve the voice handler for the guild
    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        // Determine the source based on whether it's a search or a URL
        let src = if do_search {
            YoutubeDl::new_search(http_client, args)
        } else {
            YoutubeDl::new(http_client, args)
        };

        // Inform the user that the bot is playing the input
        ctx.say("Playing").await?;
        handler.play_input(src.clone().into());
    } else {
        ctx.say("You need to be in a Voice Channel to use this command.").await?;
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