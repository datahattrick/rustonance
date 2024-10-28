use crate::{messaging::message::check_msg, model::{Context, Error}};
use tracing::info;

#[poise::command(
    guild_only,
    slash_command,
    category = "Utility"
)]
/// Leaves the current voice channel
pub async fn leave(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = match ctx.guild_id() {
        Some(guild_id) => guild_id,
        None => {
            check_msg(ctx.say("This command can only be used in a guild.").await);
            return Ok(());
        }
    };

    if let Some(handler_lock) = ctx.data().songbird.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        match handler.leave().await {
            Ok(_) => {
                info!("Successfully left the voice channel.");
                handler.remove_all_global_events(); // Remove any registered events
                handler.queue().stop();
                
                check_msg(ctx.say("Left the voice channel.").await);
            }
            Err(err) => {
                // Log the error and inform the user
                info!("Failed to leave the voice channel: {:?}", err);
                check_msg(ctx.say("Failed to leave the voice channel, please try again.").await);
            }
        }
    } else {
        check_msg(ctx.say("Not in a voice channel to leave.").await);
    }

    Ok(())
}
