use crate::{messaging::message::check_msg, utils::{Context, Error}};


#[poise::command(prefix_command, guild_only)]
pub async fn leave(ctx: Context<'_>) -> Result<(), Error>{
    let guild_id = ctx.guild_id().unwrap();


    if let Some(handler_lock) = ctx.data().songbird.get(guild_id) {
        let mut handler = handler_lock.lock().await;
    
        let _ = handler.leave().await;
        handler.remove_all_global_events();
        check_msg(
            ctx.say("Left the voice channel")
                .await,
            );
    } else {
        check_msg(ctx.say("Not in a voice channel to leave").await);
    }
    Ok(())
}