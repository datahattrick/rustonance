use std::collections::HashMap;
use std::sync::Arc;
use reqwest::Client;
use songbird::id::GuildId;
use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChannelID( pub u64);

impl From<songbird::id::ChannelId> for ChannelID {
    fn from(value: songbird::id::ChannelId) -> Self {
        self::ChannelID(value.0.get())
    }
}

impl From<poise::serenity_prelude::ChannelId> for ChannelID {
    fn from(value: poise::serenity_prelude::ChannelId) -> Self {
        self::ChannelID(value.get())
    }
}

impl PartialEq<poise::serenity_prelude::ChannelId> for ChannelID {
    fn eq(&self, other: &poise::serenity_prelude::ChannelId) -> bool {
        self.0 == other.get()
    }
}

impl ChannelID {
    pub fn get(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct UserID(pub u64);

impl From<serenity::all::UserId> for UserID {
    fn from(value: serenity::all::UserId) -> Self {
        self::UserID(value.get())
    }
}

impl From<songbird::id::UserId> for UserID {
    fn from(value: songbird::id::UserId) -> Self {
        self::UserID(value.0.get())
    }
}

impl UserID {
    pub fn get(self) -> u64 {
        self.0
    }
}


#[derive(Debug, Deserialize, Serialize)]
pub struct ChannelData {
    pub bot_id: UserID,
    pub channel_id: ChannelID,
    pub user_count: HashMap<ChannelID, usize>,
}

pub struct AsyncChannelData {
    pub channel_data: Mutex<ChannelData>
}

impl AsyncChannelData {
    pub fn new(bot_id: UserID, channel_id: ChannelID) -> Self {
        AsyncChannelData {
            channel_data: Mutex::new(ChannelData {
                bot_id,
                channel_id,
                user_count: HashMap::new(),
            }),
        }
    }
    pub async fn increment_user_count(&self, channel_id: ChannelID) {
        let mut channel_data = self.channel_data.lock().await;
        *channel_data.user_count.entry(channel_id).or_insert(0) += 1;
    }

    pub async fn decrement_user_count(&self, channel_id: ChannelID) {
        let mut channel_data = self.channel_data.lock().await;
        if let Some(count) = channel_data.user_count.get_mut(&channel_id) {
            if *count > 0 {
                *count -= 1;
            }
        }
    }

    pub async fn to_serializable(&self) -> ChannelData {
        let inner = self.channel_data.lock().await;
        ChannelData {
            bot_id: inner.bot_id,
            channel_id: inner.channel_id,
            user_count: inner.user_count.clone(),
        }
    }

}

// Custom user data passed to all command functions
pub struct UserData {
    pub http_client: Client,
    pub songbird: Arc<songbird::Songbird>,
    pub channel: AsyncChannelData,
    pub guild_id: GuildId
}

impl UserData {
    pub fn http_client(&self) -> &Client {
        &self.http_client
    }
    pub fn songbird(&self) -> &Arc<songbird::Songbird> {
        &self.songbird
    }
    pub fn channel(&self) -> &AsyncChannelData{
        &self.channel
    }

    pub async fn to_json(&self) -> Result<String, serde_json::Error> {
        let serializable_channel = self.channel.to_serializable().await;
        serde_json::to_string(&serializable_channel)
    }
}


// Types used by all command functions
pub type Error = Box<dyn std::error::Error + Send + Sync>;
#[allow(unused)]
pub type Context<'a> = poise::Context<'a, UserData, Error>;

pub struct TrackInfo {
    pub name: String,
    pub artists: Vec<String>,
    pub duration: u64,
    pub image_url: String
}

pub enum Tracks {
   FullTrack,
   Track,
}