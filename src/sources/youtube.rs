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
        let metadata = vid.aux_metadata().await
            .map_err(|err| error!("Failed to get video metadata: {:?}", err)).unwrap();
        TrackInfo {
            name: metadata.title.unwrap(),
            artists: vec![metadata.artist.unwrap()],
            duration: metadata.duration.unwrap().as_secs(),
            image_url: metadata.thumbnail.unwrap() 
        }
    }
}