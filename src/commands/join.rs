use ::std::time::Duration;

use crate::{handlers::{idle::IdleHandler, serenity::TrackErrorNotifier}, messaging::message::check_msg, model::{Context, Error}};
use ::songbird::Event;
use songbird::TrackEvent;


#[poise::command( 
    guild_only,
    slash_command, 
    category = "Utility")]
/// The join command, brings the discord bot into the voice channel
pub async fn join(ctx: Context<'_>,
) -> Result<(), Error>{
    join_channel(ctx).await
}

pub async fn join_channel(
    ctx: Context<'_>
) ->  Result<(),Error> {
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
    if let Some(call) = manager.get(guild_id) {
        let handler = call.lock().await;

        if let Some(current_channel) = handler.current_channel() {
            if handler.queue().is_empty() {
                manager.join(guild_id, connect_to).await?;
                check_msg(ctx.say("On my way!").await);
            } else if current_channel != connect_to.into() {
                check_msg(ctx.reply("Bot is already in use, cannot join another channel.").await);
                return Ok(());
            }
        } else {
            manager.join(guild_id, connect_to).await?;   
            check_msg(ctx.say("On my way!").await);     
        }
    } else {
        manager.join(guild_id, connect_to).await?;
        check_msg(ctx.say("On my way!").await);
    }

    // Now get the call handler to attach event handlers
    if let Some(call) = manager.get(guild_id) {
        let mut handler = call.lock().await;
        handler.remove_all_global_events();

        // Add a TrackErrorNotifier to monitor track errors
        handler.add_global_event(TrackEvent::Error.into(), TrackErrorNotifier);
        
        const DEFAULT_IDLE_TIMEOUT: usize = 60 * 5;

        let idle_timeout = std::env::var("IDLE_TIMEOUT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(DEFAULT_IDLE_TIMEOUT);

        handler.add_global_event(
            Event::Periodic(Duration::from_secs(1), None),
            IdleHandler {
                manager: manager.clone(),
                guild_id: ctx.guild_id().unwrap(),
                limit: idle_timeout,
                count: Default::default()
            },
        );
    }
    Ok(())
}