use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SocialPhotoSize {
    pub url: String,
    pub width: Option<String>,
    pub height: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SocialPhoto {
    pub social_id: Option<String>,
    pub sizes: Vec<SocialPhotoSize>
}
