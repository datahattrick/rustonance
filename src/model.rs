


pub struct TrackInfo {
    pub name: String,
    pub artists: Vec<String>,
    pub duration: u64,
    pub image_url: String
}

pub enum Tracks {
   FullTrack,
   Track,
}
