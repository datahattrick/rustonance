
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::gateway::Ready,
};

pub struct SerenityHandler;

#[async_trait]
impl EventHandler for SerenityHandler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected", ready.user.name)
    }
}