
use ::serenity::all::{ChannelId, Context, CreateEmbed, CreateEmbedFooter, CreateMessage, Timestamp};
use tracing::error;

/// Checks that a message successfully sent; if not, then logs why to stdout.
pub fn check_msg<T>(result: serenity::Result<T>) {
    if let Err(why) = result {
        error!("Error sending message: {:?}", why);
    }
}

fn format_time(minutes: u64) -> String {
    let hours = minutes / 60;
    let minutes = minutes % 60;
    format!("{:02}:{:02}", hours, minutes)
}

pub fn create_music_embed(title: String, artist: String, duration: u64, queue: usize, image_url: String) -> CreateEmbed {
    let embed = CreateEmbed::default();
    let footer = CreateEmbedFooter::new("Queue: ".to_string() + &queue.to_string());

    embed.title("Now Playing")
        .field("Title", title, false)
        .field("Artist", artist, false)
        .image(image_url)
        .field("Duration", format_time(duration), false)
        .color(serenity::model::Colour::DARK_BLUE)
        .footer(footer)
        .timestamp(Timestamp::now())
} 


pub async fn create_music_message(embed: CreateEmbed) -> CreateMessage {
    CreateMessage::new()
    .embed(embed)
}

pub async fn send_music_message(ctx: &Context, channel_id: ChannelId, message: CreateMessage) {
    if let Err(e) = channel_id.send_message(&ctx.http, message)
    .await {
        println!("Error sending message: {:?}", e);
    }
}