use std::sync::Arc;
use reqwest::Client;
use tokio::sync::Mutex;
use serde::Serialize;
use serde::ser::{Serialize, Serializer, SerializeStruct};

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

#[derive(Debug, Serialize, Clone, Copy)]
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

#[derive(Debug, Serialize, Clone, Copy)]
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

#[derive(Debug)]
pub struct ChannelData {
    pub bot_id: Mutex<UserID>,
    pub channel_id: Mutex<ChannelID>,
    pub count: Mutex<usize>,
}

// A temporary struct that can be serialized
#[derive(Serialize)]
struct ChannelDataSync {
    bot_id: UserID,
    channel_id: ChannelID,
    count: usize,
}

impl ChannelData {
    // Async method to gather data and serialize
    pub async fn serialize_async<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let bot_id = self.bot_id.lock().await;
        let channel_id = self.channel_id.lock().await;
        let count = self.count.lock().await;

        let sync_data = ChannelDataSync {
            bot_id: *bot_id,
            channel_id: *channel_id,
            count: *count,
        };

        sync_data.serialize(serializer) // Serialize the temporary struct
    }
}


// Custom user data passed to all command functions
pub struct UserData {
    pub http_client: Client,
    pub songbird: Arc<songbird::Songbird>,
    pub channel: ChannelData
}

// Custom serialization for UserData
impl UserData {
    pub async fn serialize_async<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Create the serializer state with 1 serializable field ("channel")
        let mut state = serializer.serialize_struct("UserData", 1)?;

        // Manually serialize the `channel` field using its async serialization
        state.serialize_field("channel", &self.channel.serialize_async(serializer).await)?;

        state.end()
    }
}

impl UserData {
    pub fn http_client(&self) -> &Client {
        &self.http_client
    }
    pub fn songbird(&self) -> Arc<songbird::Songbird> {
        self.songbird.clone()
    }
    pub fn channel(&self) -> &ChannelData{
        &self.channel
    }
}


// Types used by all command functions
pub type Error = Box<dyn std::error::Error + Send + Sync>;
#[allow(unused)]
pub type Context<'a> = poise::Context<'a, UserData, Error>;