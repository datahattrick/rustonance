use songbird::input::YoutubeDl;
use crate::utils::{Context, Error};
use poise::serenity_prelude as serenity;

// Event related imports to detect track creation failures.
use songbird::events::{Event, EventContext, EventHandler as VoiceEventHandler, TrackEvent};
use serenity::async_trait;

//use crate::sources::spotify;

type CommandResult = Result<(), Error>;

#[derive(Clone)]
pub enum QueryType {
    Keywords(String),
    KeywordList(Vec<String>),
    VideoLink(String),
    PlaylistLink(String),
}

#[poise::command(prefix_command, guild_only)]
pub async fn join(ctx: Context<'_>) -> CommandResult {
    let (guild_id, channel_id) = {
        let guild = ctx.guild().unwrap();
        let channel_id = guild
            .voice_states
            .get(&ctx.author().id)
            .and_then(|voice_state| voice_state.channel_id);

        (guild.id, channel_id)
    };

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            check_msg(ctx.reply("Not in a voice channel").await);
            return Ok(());
        },
    };

    let manager = &ctx.data().songbird;
    if let Ok(handler_lock) = manager.join(guild_id, connect_to).await {
        // Attach an event handler to see notifications of all track errors.
        let mut handler = handler_lock.lock().await;
        handler.add_global_event(TrackEvent::Error.into(), TrackErrorNotifier);
    }

    Ok(())
}

#[poise::command(prefix_command, guild_only)]
pub async fn play(ctx: Context<'_>, url: String) -> CommandResult {
    let do_search = !url.starts_with("http");

    let guild_id = ctx.guild_id().unwrap();
    let data = ctx.data();

    if let Some(handler_lock) = data.songbird.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let src = if do_search {
            YoutubeDl::new_search(data.http_client.clone(), url)
        } else {
            YoutubeDl::new(data.http_client.clone(), url)
        };
        let _ = handler.play_input(src.into());

        check_msg(ctx.say("Playing song").await);
    } else {
        check_msg(ctx.say("Not in a voice channel to play in").await);
    }

    Ok(())
}

struct TrackErrorNotifier;

#[async_trait]
impl VoiceEventHandler for TrackErrorNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(track_list) = ctx {
            for (state, handle) in *track_list {
                println!(
                    "Track {:?} encountered an error: {:?}",
                    handle.uuid(),
                    state.playing
                );
            }
        }
        None
    }
}

/// Checks that a message successfully sent; if not, then logs why to stdout.
fn check_msg<T>(result: serenity::Result<T>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}