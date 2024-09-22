

use poise::serenity_prelude as serenity;
use ::serenity::{all::ChannelId, async_trait};
use songbird::events::{Event, EventContext, EventHandler as VoiceEventHandler};
use tracing::error;
use ::tracing::info;
use crate::{messaging::message::{create_music_embed, create_music_message, send_music_message}, model::{Error, UserData as Data}};


pub async fn event_handler(
    _ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    // Handle the Ready event when the bot logs in
    if let serenity::FullEvent::Ready { data_about_bot, .. } = event {
        info!("Logged in as {}", data_about_bot.user.name);

        // Update the bot ID in shared state
        let mut update_bot_id = data.channel.bot_id.lock().await;
        *update_bot_id = data_about_bot.user.id.into();
    }

    // Handle the VoiceStateUpdate event when someone joins or leaves a voice channel
    if let serenity::FullEvent::VoiceStateUpdate { new, .. } = event {
        let bot_id = _ctx.http.get_current_user().await.unwrap().id;

        // Check if the bot joined a channel
        if let Some(user) = &new.member {
            if bot_id == user.user.id {
                if let Some(guild_id) = new.guild_id {
                    // Check for a Songbird handler (for voice channel interactions)
                    if let Some(handler_lock) = data.songbird().get(guild_id) {
                        let handler = handler_lock.lock().await;

                        // Check if the bot is in a voice channel
                        if let Some(channel_id) = handler.current_channel() {
                            info!("Updating channel ID: {}", channel_id);

                            // Update the channel ID in shared state
                            let mut update_channel_id = data.channel.channel_id.lock().await;
                            *update_channel_id = channel_id.into();

                            // Fetch the number of users in the channel
                            let poise_channel_id = poise::serenity_prelude::ChannelId::new(channel_id.0.get());
                                    // Get the number of users in the voice channel
                            if let Some(voice_states) = _ctx.cache.guild(guild_id).map(|guild| guild.voice_states.clone()) {
                                let member_count = voice_states
                                    .values()
                                    .filter(|vs| vs.channel_id == Some(poise_channel_id))
                                    .count();

                                let mut set_count = data.channel.count.lock().await;
                                info!("There is {} members in this channel", member_count.clone());
                                *set_count = member_count;
                            }
                        } else {
                            info!("Bot is not in a voice channel.");
                        }
                    } else {
                        info!("No Songbird handler found for the guild.");
                    }
                } else {
                    info!("Guild ID is missing from VoiceStateUpdate.");
                }
            } else {
                // Handle when another user joins the same channel as the bot
                let current_channel_id = data.channel.channel_id.lock().await;
                if let Some(channel_id) = new.channel_id {
                    if channel_id.get() == current_channel_id.0 {
                        // Increment the member count
                        let mut count = data.channel.count.lock().await;
                        *count += 1;
                    }
                } else {
                    info!("Failed to get channel ID from event.");
                }
            }
        } else {
            info!("Member data is missing from the user.");
        }
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
    pub ctx: serenity::Context
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
