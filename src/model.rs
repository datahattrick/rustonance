use std::sync::Arc;
use reqwest::Client;
use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};
use serde::ser::{Serializer, SerializeStruct};

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

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
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

#[derive(Debug, Deserialize, Serialize)]
pub struct ChannelDataSerializable {
    pub bot_id: UserID,
    pub channel_id: ChannelID,
    pub count: usize
}

pub struct ChannelData {
    pub bot_id: Mutex<UserID>,
    pub channel_id: Mutex<ChannelID>,
    pub count: Mutex<usize>,
}

impl ChannelData {
    pub async fn to_serializable(&self) -> ChannelDataSerializable {
        ChannelDataSerializable {
            bot_id: *self.bot_id.lock().await,
            channel_id: *self.channel_id.lock().await,
            count: *self.count.lock().await,
        }
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
        let channel_data_serializable = self.channel.to_serializable().await;

        let mut state = serializer.serialize_struct("UserData", 1)?;
        state.serialize_field("channel", &channel_data_serializable)?;
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