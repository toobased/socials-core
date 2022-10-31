use vk_client::attachments::models::{Attachment, VkPhoto, VkPhotoSize};

use crate::social::attachments::{SocialAttachmentType, photo::{SocialPhoto, SocialPhotoSize}};

impl From<VkPhotoSize> for SocialPhotoSize {
    fn from(v: VkPhotoSize) -> Self {
        let w: u32 = v.height.unwrap_or(0);
        let h: u32 = v.height.unwrap_or(0);
        SocialPhotoSize {
            url: v.url,
            width: Some(w.to_string()),
            height: Some(h.to_string())
        }
    }
}

impl From<VkPhoto> for SocialPhoto {
    fn from(v: VkPhoto) -> Self {
        SocialPhoto {
            social_id: Some(v.id.to_string()),
            sizes: v.sizes.into_iter().map(|v| { v.into() }).collect(),
            ..Default::default()
        }
    }
}

impl From<Attachment> for SocialAttachmentType {
    fn from(a: Attachment) -> Self {
        if let Some(v) = a.photo { return Self::SocialPhoto(SocialPhoto::from(v)) }
        Self::Dummy
    }
}
