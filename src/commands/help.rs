
use ::poise::samples::HelpConfiguration;

use crate::utils::{Context, Error};

use crate::messaging::message::check_msg;

#[poise::command(prefix_command, 
    slash_command, 
    track_edits, 
    category = "Utility")]
/// The help command, run /help {command} for more info, i.e. /help play
pub async fn help(
    ctx: Context<'_>,
    #[description = "Command to get help for"]
    #[rest]
    mut command: Option<String>,
) -> Result<(), Error> {
    // This makes it possible to just make `help` a subcommand of any command
    // `/fruit help` turns into `/help fruit`
    // `/fruit help apple` turns into `/help fruit apple`
    if ctx.invoked_command_name() != "help" {
        command = match command {
            Some(c) => Some(format!("{} {}", ctx.invoked_command_name(), c)),
            None => Some(ctx.invoked_command_name().to_string()),
        };
    }
    let extra_text_at_bottom = "\
Type `/help command` for more info on a command.
You can edit your `/help` message to the bot and the bot will edit its response.";

    let config = HelpConfiguration {
        show_subcommands: true,
        show_context_menu_commands: true,
        ephemeral: true,
        extra_text_at_bottom,

        ..Default::default()
    };
    poise::builtins::help(ctx, command.as_deref(), config).await?;
    Ok(())
}

#[poise::command(prefix_command, 
    slash_command, 
    track_edits, 
    category = "Utility")]
/// The help command, run /help {command} for more info, i.e. /help play
pub async fn channel(
    ctx: Context<'_>,
    #[description = "Debug command"]
    #[rest]
    command: Option<String>,
) -> Result<(), Error> {
    let bot = ctx.http().get_current_user().await.unwrap().to_string();
    let channel = *ctx.data().channel.lock().await.get(&bot).unwrap();
    check_msg(ctx.say(format!("{}", channel)).await);
    Ok(())
}