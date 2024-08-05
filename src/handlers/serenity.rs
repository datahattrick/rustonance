use poise::serenity_prelude as serenity;
use crate::utils::{UserData, Error};

pub async fn event_handler(
    _ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, UserData, Error>,
    _data: &UserData,
) -> Result<(), Error> {
    match *event {
        serenity::FullEvent::Ready { ref data_about_bot, .. } => {
            println!("Logged in as {}", data_about_bot.user.name);
        }
        _ => {}
    }
    Ok(())
}
