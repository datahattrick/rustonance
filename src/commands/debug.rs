

use tracing::info;

use crate::model::{Context, Error};


#[poise::command(prefix_command, 
    slash_command, 
    track_edits, 
    category = "Utility")]
/// The help command, run /help {command} for more info, i.e. /help play
pub async fn debug(
    ctx: Context<'_>,
    #[description = "Command to get help for"]
    #[rest]
    command: Option<String>,
) -> Result<(), Error> {
    // This makes it possible to just make `help` a subcommand of any command
    // `/fruit help` turns into `/help fruit`
    // `/fruit help apple` turns into `/help fruit apple`
    if let Some(command) = command {
        if command == "state" {
            ctx.say(ctx.data());
        }
    }

    Ok(())
}
