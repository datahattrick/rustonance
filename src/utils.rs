use std::{collections::HashMap, sync::Arc};

use reqwest::Client;
use songbird::{id::GuildId, tracks::TrackQueue, Driver};


// Custom user data passed to all command functions
pub struct UserData {
    pub http_client: Client,
    pub songbird: Arc<songbird::Songbird>,
    pub driver: Driver,
    pub queues: HashMap<GuildId, TrackQueue>,
}

impl UserData {
    pub fn http_client(&self) -> Client {
        self.http_client.clone()
    }
    pub fn songbird(&self) -> Arc<songbird::Songbird> {
        self.songbird.clone()
    }
}

// Types used by all command functions
pub type Error = Box<dyn std::error::Error + Send + Sync>;
#[allow(unused)]
pub type Context<'a> = poise::Context<'a, UserData, Error>;

