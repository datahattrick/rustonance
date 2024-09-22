use std::num::NonZeroU64;
use std::sync::Arc;
use reqwest::Client;
use tokio::sync::Mutex;

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

pub struct ChannelID( pub NonZeroU64);

impl From<songbird::id::ChannelId> for ChannelID {
    fn from(value: songbird::id::ChannelId) -> Self {
        value.into()
    }
}

impl From<poise::serenity_prelude::ChannelId> for ChannelID {
    fn from(value: poise::serenity_prelude::ChannelId) -> Self {
        value.into()
    }
}

impl PartialEq<poise::serenity_prelude::ChannelId> for ChannelID {
    fn eq(&self, other: &poise::serenity_prelude::ChannelId) -> bool {
        self.0.get() == other.get()
    }
    fn ne(&self, other: &poise::serenity_prelude::ChannelId) -> bool {
        self.0.get() != other.get()
    }
}

pub struct UserID(pub NonZeroU64);

impl From<serenity::all::UserId> for UserID {
    fn from(value: serenity::all::UserId) -> Self {
        value.into()
    }
}

impl From<songbird::id::UserId> for UserID {
    fn from(value: songbird::id::UserId) -> Self {
        value.into()
    }
}


pub struct ChannelData {
    pub bot_id: Mutex<UserID>,
    pub channel_id: Mutex<ChannelID>,
    pub count: Mutex<i32>,
}

// Custom user data passed to all command functions
pub struct UserData {
    pub http_client: Client,
    pub songbird: Arc<songbird::Songbird>,
    pub channel: ChannelData
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