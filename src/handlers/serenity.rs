use poise::serenity_prelude as serenity;
use ::serenity::async_trait;
use songbird::events::{Event, EventContext, EventHandler as VoiceEventHandler};
use tracing::{info, error};
use crate::utils::{UserData, Error};

pub async fn event_handler(
    _ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, UserData, Error>,
    _data: &UserData,
) -> Result<(), Error> {
    if let serenity::FullEvent::Ready { ref data_about_bot, .. } = *event {
        info!("Logged in as {}", data_about_bot.user.name);
    }
    Ok(())
}

pub struct TrackErrorNotifier;

#[async_trait]
impl VoiceEventHandler for TrackErrorNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(track_list) = ctx {
            for (state, handle) in *track_list {
                error!(
                    "Track {:?} encountered an error: {:?}",
                    handle.uuid(),
                    state.playing
                );
            }
        }
        None
    }
}
