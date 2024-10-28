


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

    if let Some(command) = command {
        if command == "state" {
            match ctx.data().to_json().await {
                Ok(json_string) => { ctx.say(format!("Hi, here is who I is: {}", json_string)).await?; }
                Err(e) => { ctx.say(format!("Sorry something went wrong: Error serializing UserData: {}", e)).await?; }
            };
        } else {
            ctx.say("Command not found").await?;
        }
    }

    Ok(())
}
