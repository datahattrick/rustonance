use std::env;
use std::sync::Arc;
use std::time::Duration;

use poise::serenity_prelude as serenity;
use songbird::SerenityInit;

use crate::commands::{
    help::help,
    music::play,
};
use crate::utils;

async fn on_error(error: poise::FrameworkError<'_, utils::Data, utils::Error>) {
    // This is our custom error handler
    // They are many errors that can occur, so we only handle the ones we want to customize
    // and forward the rest to the default handler
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx, .. } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}

pub struct Client {
    client: serenity::Client,
}

impl Client {
    pub async fn default() -> Result<Client, Box<dyn std::error::Error>> {
        let token = env::var("DISCORD_TOKEN").expect("Fatality! DISCORD_TOKEN not set!");
        Client::new(token).await
    }

    pub async fn new(token: String) -> Result<Client, Box<dyn std::error::Error>> {

        let intents = serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

        let options = poise::FrameworkOptions {
            // List of commands
            commands: vec![help(), play()],
            // What prefix to look for
            prefix_options: poise::PrefixFrameworkOptions { 
                prefix: Some("/".into()), 
                edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(Duration::from_secs(3600)))),
                additional_prefixes: vec![
                    poise::Prefix::Literal("hey bot"),
                    poise::Prefix::Literal("hey bot,"),
                ],
                ..Default::default()
            },

            // The Global error handler for all error cases that may occur
            on_error: |error| Box::pin(on_error(error)),

            pre_command: |ctx| {
                Box::pin(async move {
                    println!("Executing command {}...", ctx.command().qualified_name);
                })
            },
            // This code is run after a command if it was successful (returned Ok)
            post_command: |ctx| {
                Box::pin(async move {
                    println!("Executed command {}!", ctx.command().qualified_name);
                })
            },
            // Enforce command checks even for owners (enforced by default)
            // Set to true to bypass checks, which is useful for testing
            skip_checks_for_owners: false,
            event_handler: |ctx, event, framework, data| 
                Box::pin(event_handler(ctx, event, framework, data)),
            ..Default::default()
        };


        let framework = poise::Framework::builder()
            .setup(move |_ctx, _ready, _framework| {
                Box::pin(async move {
                    let http_client = reqwest::Client::new();
                    Ok(utils::Data { http_client })
                })
            })
            .options(options)
         .build();


        let client = serenity::Client::builder(&token, intents)
            .framework(framework)
            .register_songbird()
            .await?;

        Ok(Client { client })
    }

    pub async fn start(&mut self) -> Result<(), serenity::Error> {
        self.client
            .start()
            .await
    }
}

async fn event_handler(
    _ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, utils::Data, utils::Error>,
    _data: &utils::Data,
) -> Result<(), utils::Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            println!("Logged in as {}", data_about_bot.user.name);
        }
        _ => {}
    }
    Ok(())
}

