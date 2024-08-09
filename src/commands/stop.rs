
use crate::utils::{Error,Context};

use crate::messaging::message::check_msg;


#[poise::command(prefix_command, guild_only)]
pub async fn stop(ctx: Context<'_>) ->  Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();


    if let Some(handler_lock) = ctx.data().songbird.get(guild_id) {
        let handler = handler_lock.lock().await;
    
        let queue = handler.queue();
        queue.stop();

        check_msg(
            ctx.say("Stopped and cleared the queue".to_string(),)
                .await,
            );
    } else {
        check_msg(ctx.say("Not in a voice channel to play in").await);
    }

    Ok(())
}

