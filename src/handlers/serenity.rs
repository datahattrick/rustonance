


use poise::serenity_prelude as serenity;
use ::serenity::{all::ChannelId, async_trait};
use songbird::events::{Event, EventContext, EventHandler as VoiceEventHandler};
use tracing::error;
use ::tracing::info;
use crate::{messaging::message::{create_music_embed, create_music_message, send_music_message}, model::{ChannelID, Error, UserData as Data}};


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
        let mut update_user_data = data.channel.channel_data.lock().await;
        update_user_data.bot_id = data_about_bot.user.id.into();
    }

    
    // Handle the VoiceStateUpdate event when someone joins or leaves a voice channel
    if let serenity::FullEvent::VoiceStateUpdate { old, new } = event {
        info!("Updating voice state");
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

                            {
                            // Update the channel ID in shared state
                            let mut update_channel_id = data.channel.channel_data.lock().await;
                            update_channel_id.channel_id = channel_id.into();
                            }

                            // Fetch the number of users in the channel
                            let poise_channel_id = poise::serenity_prelude::ChannelId::new(channel_id.0.get());
                                    // Get the number of users in the voice channel
                            if let Some(voice_states) = _ctx.cache.guild(guild_id).map(|guild| guild.voice_states.clone()) {
                                let member_count = voice_states
                                    .values()
                                    .filter(|vs| vs.channel_id == Some(poise_channel_id))
                                    .count();

                                {
                                    let mut set_count = data.channel.channel_data.lock().await;
                                    info!("There is {} members in this channel", member_count.clone());
                                    set_count.user_count.insert(channel_id.into(), member_count);
                                    //*entry = member_count;
                                }
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
                let mut channel_data = data.channel.channel_data.lock().await;
                let bot_channel = channel_data.channel_id;

                let previous_channel: ChannelID = match old {
                    Some(voice_state) => voice_state.channel_id.map(|channel_id| channel_id.into()).unwrap_or(bot_channel),
                    None => bot_channel,
                };

                if let Some(channel_id) = new.channel_id {
                    if channel_id.get() == bot_channel.get() {
                        // Increment the member count
                        let count = channel_data.user_count.entry(channel_id.into()).or_insert(0);
                        *count += 1;
                        info!("User joined. Updated count: {}", *count);
                    }

                else if previous_channel.get() == bot_channel.get() {
                    // User left the channel
                    let count = channel_data.user_count.entry(bot_channel).or_insert(0);

                    if *count > 0 {
                        *count -= 1;
                        info!("Updated count after someone left: {}", *count);

                        // If only the bot is left in the channel, make it leave
                        if *count <= 1 {
                            drop(channel_data); // Release the lock before making an async call
                            if let Some(handler_lock) = data.songbird().get(new.guild_id.unwrap()) {
                                let mut handler = handler_lock.lock().await;
                                handler.remove_all_global_events();
                                info!("Everyon left");
                                if handler.leave().await.is_ok() {
                                    handler.queue().stop();
                                    info!("Bot is leaving because it's the only user left in the channel");
                                }
                            } else {
                                info!("Why didn't this work")
                            }
                        }
                    } else {
                        info!("Count is already zero; cannot decrement.");
                    }
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
