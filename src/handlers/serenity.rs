use poise::serenity_prelude as serenity;
use ::serenity::{all::{ChannelId, Context}, async_trait};
use songbird::events::{Event, EventContext, EventHandler as VoiceEventHandler};
use tracing::error;
use ::tracing::info;
use crate::{messaging::message::{create_music_embed, create_music_message, send_music_message}, utils::{Error, UserData as Data}};


pub async fn event_handler(
    _ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    if let serenity::FullEvent::Ready { data_about_bot, .. } = event {
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


pub struct NextInQueueNotifier {
    pub title: String,
    pub artists: Vec<String>,
    pub image_url: String,
    pub duration: u64,
    pub queue: usize,
    pub channel_id: ChannelId,
    pub ctx: Context
}

#[async_trait]
impl VoiceEventHandler for NextInQueueNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(_track_list) = ctx {
            return None
        }
        info!("Attemping to show next song");

        let embed = create_music_embed(self.title.clone(), 
            self.artists.join(" "), self.duration, self.queue, self.image_url.clone());
        let message = create_music_message(embed).await;
        send_music_message(&self.ctx, self.channel_id, message).await;
   
        None
        
    }
}
