use std::env;
use std::num::NonZero;
use std::sync::Arc;

use poise::serenity_prelude as serenity;
use songbird::id::GuildId;
use ::tracing::info;


use crate::commands::repeat::repeat;
use crate::commands::resume::resume;
use crate::commands::skip::skip;
use crate::handlers::serenity::event_handler;
use crate::commands::{
    help::help,
    play::play,
    join::join,
    skip::next,
    pause::pause,
    stop::stop,
    leave::leave,
    debug::debug,
    ew::ew,
};
use crate::model::{AsyncChannelData, ChannelID, Error, UserData, UserID};

// YtDl requests need an HTTP client to operate -- we'll create and store our own.
use reqwest::Client as HttpClient;

async fn on_error(error: poise::FrameworkError<'_, UserData, Error>) {
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
    pub async fn default() -> Result<Client, Error> {
        let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not set!");
        Client::new(token).await
    }

    pub async fn new(token: String) -> Result<Client, Error> {
        let intents = serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;
        let options = poise::FrameworkOptions {
            // List of commands
            commands: vec![
                help(),
                play(),
                join(),
                next(),
                skip(),
                pause(),
                stop(),
                leave(),
                resume(),
                repeat(),
                debug(),
                ew()],
            // What prefix to look for
            prefix_options: poise::PrefixFrameworkOptions { 
                prefix: Some("/".into()),
                ..Default::default()
            },

            // The Global error handler for all error cases that may occur
            on_error: |error| Box::pin(on_error(error)),

            pre_command: |ctx| {
                Box::pin(async move {
                    info!("Executing command {}...", ctx.command().qualified_name);
                })
            },
            // This code is run after a command if it was successful (returned Ok)
            post_command: |ctx| {
                Box::pin(async move {
                    info!("Executed command {}!", ctx.command().qualified_name);
                })
            },
            // Enforce command checks even for owners (enforced by default)
            // Set to true to bypass checks, which is useful for testing
            skip_checks_for_owners: false,
            event_handler: |ctx, event, framework, data| 
                Box::pin(event_handler(ctx, event, framework, data)),
            ..Default::default()
        };

        let manager = songbird::Songbird::serenity();
        let manager_clone = Arc::clone(&manager);

        let framework = poise::Framework::builder()
            .setup(move |ctx, _ready, framework| {
                Box::pin(async move {
                    poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                    Ok( UserData { 
                        http_client: HttpClient::new(),
                        songbird: manager_clone,
                        channel:  AsyncChannelData::new(UserID(1), ChannelID(1)),
                        guild_id: GuildId(NonZero::new(1).unwrap())
                     })
                })
            })
            .options(options)
         .build();

        let client = serenity::Client::builder(&token, intents)
            .voice_manager_arc(manager)
            .framework(framework)
            .await
            .expect("Err creating client");

        Ok(Client { client })
    }

    pub async fn start(&mut self) -> Result<(), serenity::Error> {
        self.client
            .start()
            .await
    }
}
