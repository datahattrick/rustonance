use crate::{model::TrackInfo, utils::Context};
use songbird::input::{Compose, YoutubeDl};
use tracing::error;

pub struct YouTube {}

impl YouTube {
    pub async fn query(ctx: Context<'_>, query: String) -> Result<YoutubeDl, ()> {
        let do_search = !query.starts_with("http");

        let src = if do_search {
            YoutubeDl::new_search(ctx.data().http_client.clone(), query)
        } else {
            YoutubeDl::new(ctx.data().http_client.clone(), query)
        };
        Ok(src)
    }

    pub async fn info(mut vid: YoutubeDl) -> TrackInfo {
        match vid.aux_metadata().await {
            Ok(metadata) => TrackInfo {
                name: metadata.title.unwrap(),
                artists: vec![metadata.artist.unwrap()],
                duration: metadata.duration.unwrap().as_secs(),
                image_url: metadata.thumbnail.unwrap() 
            },
            Err(_) => {
                error!("Failed to get video metadata");
                TrackInfo {
                    name: "Track".to_string(),
                    artists: vec!["Artist".to_string()],
                    duration: 0,
                    image_url: "https://imgur.com/hAV6F86".to_string()
                }
            }
        }
    }
}
