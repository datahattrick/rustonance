use std::{collections::HashMap, sync::Arc};

use reqwest::Client;
use songbird::id::ChannelId;
use tokio::sync::Mutex;


// Custom user data passed to all command functions
pub struct UserData {
    pub http_client: Client,
    pub songbird: Arc<songbird::Songbird>,
    pub channel: Mutex<HashMap<String,ChannelId>>,
}

impl UserData {
    pub fn http_client(&self) -> &Client {
        &self.http_client
    }
    pub fn songbird(&self) -> Arc<songbird::Songbird> {
        self.songbird.clone()
    }
    pub fn channel(&self) -> &Mutex<HashMap<String, ChannelId>> {
        &self.channel
    }
}

// Types used by all command functions
pub type Error = Box<dyn std::error::Error + Send + Sync>;
#[allow(unused)]
pub type Context<'a> = poise::Context<'a, UserData, Error>;
