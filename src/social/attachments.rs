use serde::{Serialize, Deserialize};

pub mod photo;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SocialAttachmentType {
    SocialPhoto(photo::SocialPhoto),
    Dummy
}
