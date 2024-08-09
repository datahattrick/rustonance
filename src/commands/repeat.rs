
use ::songbird::tracks::{LoopState, TrackHandle};

use crate::utils::{Error,Context};

use crate::messaging::message::check_msg;


#[poise::command(prefix_command, guild_only)]
pub async fn repeat(ctx: Context<'_>) ->  Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();


    if let Some(handler_lock) = ctx.data().songbird.get(guild_id) {
        let handler = handler_lock.lock().await;
    
        let track = handler.queue().current().unwrap();

        let was_looping = track.get_info().await.unwrap().loops == LoopState::Infinite;
        let toggle = if was_looping {
            TrackHandle::disable_loop
        } else {
            TrackHandle::enable_loop
        };

        match toggle(&track) {
            Ok(_) if was_looping => {
                check_msg(ctx.say("Disabling the loop!").await)
            }
            Ok(_) if !was_looping => {
                check_msg(ctx.say("Repeating, remember to use /repeat again to disable").await)
            }
            _ => check_msg(ctx.say("Sorry something went wrong").await)
        }
    } else {
        check_msg(ctx.say("Not in a voice channel to play in").await);
    }

    Ok(())
}

