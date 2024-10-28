use std::time::Duration;

use crate::{
    handlers::{idle::IdleHandler, serenity::TrackErrorNotifier},
    messaging::message::check_msg,
    model::{Context, Error},
};
use songbird::{Event, TrackEvent};

#[poise::command(
    guild_only,
    slash_command,
    category = "Utility"
)]
/// The join command, brings the discord bot into the voice channel
pub async fn join(ctx: Context<'_>) -> Result<(), Error> {
    join_channel(ctx).await
}

pub async fn join_channel(ctx: Context<'_>) -> Result<(), Error> {
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
            check_msg(ctx.reply("Not in a voice channel").await);
            return Ok(());
        },
    };

    let manager = &ctx.data().songbird;
    
    if manager.get(guild_id).is_some() {
        manager.join(guild_id, connect_to).await?;
    } else {
        // Step 2: If no handler is found, join the new channel
        check_msg(ctx.say("On my way!").await);
        manager.join(guild_id, connect_to).await?;
    }

    // Step 3: Get the call handler to attach event handlers
    if let Some(call) = manager.get(guild_id) {
        let mut handler = call.lock().await;
        handler.remove_all_global_events();

        // Add a TrackErrorNotifier to monitor track errors
        handler.add_global_event(TrackEvent::Error.into(), TrackErrorNotifier);

        // Set IdleHandler for auto-disconnect after being idle for a specified duration
        const DEFAULT_IDLE_TIMEOUT: usize = 60 * 5;
        let idle_timeout = std::env::var("IDLE_TIMEOUT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(DEFAULT_IDLE_TIMEOUT);

        handler.add_global_event(
            Event::Periodic(Duration::from_secs(1), None),
            IdleHandler {
                manager: manager.clone(),
                guild_id,
                limit: idle_timeout,
                count: Default::default(),
            },
        );
    }
    Ok(())
}
