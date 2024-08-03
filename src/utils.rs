use reqwest::Client;


// Custom user data passed to all command functions
pub struct Data {
    pub http_client: Client,
}

impl Data {
    pub fn http_client(&self) -> Client {
        self.http_client.clone()
    }
}

// Types used by all command functions
pub type Error = Box<dyn std::error::Error + Send + Sync>;
#[allow(unused)]
pub type Context<'a> = poise::Context<'a, Data, Error>;