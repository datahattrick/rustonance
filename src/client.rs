use std::error::Error;
use std::env;

use reqwest::Client as HttpClient;

use serenity::{
    prelude::{GatewayIntents, TypeMapKey},
};
use songbird::SerenityInit;

use crate::handlers::SerenityHandler;

struct HttpKey;

impl TypeMapKey for HttpKey {
    type Value = HttpClient;
}

pub struct Client {
    client: serenity::Client,
}

impl Client {
    pub async fn default() -> Result<Client, Box<dyn Error>> {
        let token = env::var("DISCORD_TOKEN").expect("Fatality! DISCORD_TOKEN not set!");
        Client::new(token).await
    }

    pub async fn new(token: String) -> Result<Client, Box<dyn Error>> {

        let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

        let client = serenity::Client::builder(&token, intents)
            .event_handler(SerenityHandler)
            .register_songbird()
            .type_map_insert::<HttpKey>(HttpClient::new())
            .await?;

        Ok(Client { client })
    }

    pub async fn start(&mut self) -> Result<(), serenity::Error> {
        self.client
            .start()
            .await
    }
}