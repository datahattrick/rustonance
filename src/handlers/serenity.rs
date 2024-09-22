

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
    if let serenity::FullEvent::Ready { data_about_bot, .. } = event {
        info!("Logged in as {}", data_about_bot.user.name);
        let mut update_bot_id= data.channel.bot_id.lock().await;
        *update_bot_id = data_about_bot.user.id.into();
        // figure out how to hold data about bot
    }

    if let serenity::FullEvent::VoiceStateUpdate { new, .. } = event {
        let bot_id = _ctx.http.get_current_user().await.unwrap().id;
        // If it is the bot that joined a channel, update what channel the bot is in
        if let Some(user) = new.member.as_ref() {
            if bot_id == user.user.id {
                if let Some(guild_id) = new.guild_id {
                    // Check if we get a songbird handler for the guild
                    if let Some(handler_lock) = data.songbird().get(guild_id) {
                        let handler = handler_lock.lock().await;
                        // Safely check if the bot is in a voice channel
                        if let Some(channel_id) = handler.current_channel() {
                            // Acquire lock on the channel data and insert if necessary
                            info!("updating channel id, {}", channel_id.to_string()); 
                            {
                                info!("for user {}", bot_id.to_string());
                                let mut update_channel_id = data.channel.channel_id.lock().await; 
                                *update_channel_id = channel_id.into();
                            }
                        } else {
                            info!("Bot is not currently in a voice channel.");
                      }
                    } else {
                        info!("No Songbird handler found for the guild.");
                    }
                } else {
                    info!("Guild ID is missing from VoiceStateUpdate.");
                }  
            } else {
            // Check if the user who is not the bot joined the same channel as the bot
                let ch = data.channel.channel_id.lock().await;

                if let Some(channel_id) = new.channel_id {
                    if channel_id.get() == ch.0.get() {
                        // If use joins channel then add the amount of users by 1
                        let mut count = data.channel.count.lock().await;
                        *count += 1
                    } 
                } else {
                    info!("Failed to get channel id from event")
                }

            }
        } else {
            info!("Member data is missing from the user")
        }

    };

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
