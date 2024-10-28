use crate::messaging::message::check_msg;

use crate::model::{Context, Error};

#[poise::command(prefix_command, 
    slash_command, 
    track_edits, 
    category = "Funny")]
/// The help command, run /help {command} for more info, i.e. /help play
pub async fn ew(
    ctx: Context<'_>
) -> Result<(), Error> {

    check_msg(ctx.say("https://media1.tenor.com/m/7PMoRLOVxNcAAAAC/brother-eww-eww.gif").await);

    Ok(())
}
