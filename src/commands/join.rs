use std::error::Error;

use crate::{handlers::serenity::TrackErrorNotifier, messaging::message::check_msg, utils::Context};
use songbird::TrackEvent;


#[poise::command(prefix_command, guild_only)]
pub async fn join(ctx: Context<'_>) -> Result<(), Box<dyn Error + Send + Sync + 'static>>{
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

    // TODO: How do I want to deal with if bot already in another channel?
    // Just move like it will now or prevent in case someone else is using it?
    // While playing?

    if let Ok(handler_lock) = manager.join(guild_id, connect_to).await {
        // Attach an event handler to see notifications of all track errors.
        let mut handler = handler_lock.lock().await;
        handler.add_global_event(TrackEvent::Error.into(), TrackErrorNotifier);
    }

    if let Some(call) = manager.get(guild_id) {
        let mut handler = call.lock().await;

        handler.remove_all_global_events();

    }
    Ok(())

}