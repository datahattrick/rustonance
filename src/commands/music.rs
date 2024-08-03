
use songbird::input::YoutubeDl;

use crate::utils::{Context, Error};

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn play(ctx: Context<'_>,
    #[description = "Play a Song"] args: String
) -> Result<(), Error> {
    let do_search = !args.starts_with("http");
    let guild_id = ctx.guild_id().unwrap();

    let http_client = ctx.data().http_client();

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;
        
        let src = if do_search {
            YoutubeDl::new_search(http_client, args)
        } else {
            YoutubeDl::new(http_client, args)
        };
        let _ = handler.play_input(src.clone().into());

        ctx.say(format!("Playing song 'song'")).await?;
    } else {
        ctx.say(format!("Failed to play song")).await?;
    }
    Ok(())

}