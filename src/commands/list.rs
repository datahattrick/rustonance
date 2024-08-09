
use crate::utils::{Error,Context};

use crate::messaging::message::check_msg;


#[poise::command(prefix_command, guild_only)]
pub async fn list(ctx: Context<'_>) ->  Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();


    if let Some(handler_lock) = ctx.data().songbird.get(guild_id) {
        let handler = handler_lock.lock().await;
        
        let queue = handler.queue();
        let list = queue.current_queue();

        let song_list: Vec<String> = for each in list {
            let song_info = each.get_info().await.unwrap();
            song_info.

        }

        check_msg(
            ctx.say(format!("Current queue: {}", list))
                .await,
            );
    } else {
        check_msg(ctx.say("Not in a voice channel to play in").await);
    }

    Ok(())
}

