


use poise::serenity_prelude as serenity;
use ::serenity::{all::ChannelId, async_trait};
use songbird::events::{Event, EventContext, EventHandler as VoiceEventHandler};
use tracing::error;
use ::tracing::{info, debug};
use crate::{messaging::message::{create_music_embed, create_music_message, send_music_message}, model::{ChannelData, ChannelID, Error, UserData as Data}};


pub async fn event_handler(
    _ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready {data_about_bot, ..} => handle_ready_event(data_about_bot, data).await,
        serenity::FullEvent::VoiceStateUpdate { old, new } => handle_voice_state_update(_ctx, old, new, data).await,
        _ => Ok(()),
    }
}

async fn handle_ready_event(data_about_bot:  &serenity::Ready, data: &Data) -> Result<(), Error> {
    info!("Logged in as {}", data_about_bot.user.name);
    
    let mut update_user_data = data.channel.channel_data.lock().await;
    update_user_data.bot_id = data_about_bot.user.id.into();

    Ok(())
}

async fn handle_voice_state_update(
    ctx: &serenity::Context,
    old: &Option<serenity::VoiceState>,
    new: &serenity::VoiceState,
    data: &Data,
) -> Result<(), Error> {
    info!("Updating voice state");
    if let Ok(bot_id) = ctx.http.get_current_user().await.map(|user| user.id) {
        if let Some(user) = &new.member {
            if bot_id == user.user.id {
                debug!("bot_id: {}, has moved", bot_id);
                handle_bot_voice_update(ctx, new, data).await?;
            } else {
                debug!("User: {}, has moved", user.user.id);
                handle_user_voice_update(old, new, data).await?;
            }
        }
    }

    Ok(())
}

async fn handle_bot_voice_update(
    ctx: &serenity::Context, 
    new: &serenity::VoiceState, 
    data: &Data,
) -> Result<(), Error> {
    if let Some(guild_id) = new.guild_id {
        if let Some(handler_lock) = data.songbird().get(guild_id) {
            let handler = handler_lock.lock().await;

            if let Some(channel_id) = handler.current_channel() {
                update_channel_id(data, channel_id).await;
                update_member_count(ctx, guild_id, channel_id, data).await;
            } else {
                info!("Bot is not in a voice channel, probably because it disconnected");
                let mut channel_data = data.channel.channel_data.lock().await;
                channel_data.channel_id.0 = 1;
            }
        } else {
            info!("No Songbird handler found in the guild.");
        }
    } else {
        info!("Guild ID is missing from VoiceStateUpdate.");
    }

    Ok(())
}

async fn update_channel_id(data: &Data, channel_id: songbird::id::ChannelId) {
    info!("Updating Channel ID: {}", channel_id );
    let mut update_channel_id = data.channel.channel_data.lock().await;
    update_channel_id.channel_id = channel_id.into();
}

// This function sets the amount of users within the channel
// that the bot is joining.
async fn update_member_count(
    ctx: &serenity::Context, 
    guild_id: serenity::GuildId, 
    channel_id: songbird::id::ChannelId, 
    data: &Data
) {
    let poise_channel_id = poise::serenity_prelude::ChannelId::new(channel_id.0.get());
    if let Some(voice_states) = ctx.cache.guild(guild_id).map(|guild| guild.voice_states.clone()) {
        let member_count = voice_states
            .values()
            .filter(|vs| vs.channel_id == Some(poise_channel_id))
            .count();
        let mut set_count = data.channel.channel_data.lock().await;
        info!("There is {} members in this channel", member_count);
        set_count.user_count.insert(channel_id.into(), member_count);
    }
}

async fn handle_user_voice_update(
    old: &Option<serenity::VoiceState>,
    new: &serenity::VoiceState,
    data: &Data,
) -> Result<(), Error> {
    let mut channel_data = data.channel.channel_data.lock().await;
    let bot_channel = channel_data.channel_id;

    let previous_channel: ChannelID = match old {
        Some(voice_state) => voice_state.channel_id.map(|channel_id| channel_id.into()).unwrap_or(bot_channel),
        None => bot_channel,
    };

    if let Some(channel_id) = new.channel_id {
        if channel_id.get() == bot_channel.get() {
            increment_member_count(&mut channel_data, channel_id);
        } else if previous_channel.get() == bot_channel.get() {
            decrement_member_count( &mut channel_data, bot_channel, new.guild_id, data).await;
        }
    } else {
        decrement_member_count(&mut channel_data, bot_channel, new.guild_id, data).await;
    }

    Ok(())
}

fn increment_member_count(channel_data: &mut ChannelData, channel_id: ChannelId) {
    let count = channel_data.user_count.entry(channel_id.into()).or_insert(0);
    *count += 1;
    info!("User joined. Updated count: {}", *count)
}

async fn decrement_member_count(
    channel_data: &mut ChannelData,
    bot_channel: ChannelID,
    guild_id: Option<serenity::GuildId>,
    data: &Data,
) {
    let count = channel_data.user_count.entry(bot_channel).or_insert(0);
    if *count > 0 {
        *count -= 1;
        info!("Updated count after someone left: {}", *count);
        if *count <= 1 {
            handle_bot_leave(guild_id, data, channel_data).await;
        }
    }
}

async fn handle_bot_leave(
    guild_id: Option<serenity::GuildId>, 
    data: &Data,
    channel_data: &mut ChannelData
) {
    info!("Clearing hashmap");
    // Clear the user count hashmap
    channel_data.user_count.clear();
    info!("User count hashmap cleared");
                
    if let Some(guild_id) = guild_id {
        if let Some(handler_lock) = data.songbird().get(guild_id) {
            let mut handler = handler_lock.lock().await;
            handler.remove_all_global_events();

            if handler.leave().await.is_ok() {
                handler.queue().stop();
                info!("Bot is leaving because it's the only user left in the channel");

            }
        }
    }
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
    pub ctx: serenity::Context,
}

#[async_trait]
impl VoiceEventHandler for NextInQueueNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(_track_list) = ctx {
            return None;
        }
        info!("Attempting to show next song");

        let embed = create_music_embed(
            self.title.clone(),
            self.artists.join(" "),
            self.duration,
            self.queue,
            self.image_url.clone(),
        );
        let message = create_music_message(embed).await;
        send_music_message(&self.ctx, self.channel_id, message).await;

        None
    }
}