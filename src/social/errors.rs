use core::fmt;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SocialErrorKind { Dummy, Common, Post, NotImplemented }

impl Default for SocialErrorKind { fn default () -> Self { Self::Dummy } }

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SocialError {
    pub kind: SocialErrorKind,
    pub msg: String,
    pub detail_msg: String,
}

impl SocialError {
    pub fn new(kind: SocialErrorKind, msg: Option<&str>, detailed: Option<&str>) -> Self {
        let msg = msg.unwrap_or("").to_string();
        let detailed = detailed.unwrap_or("").to_string();
        Self { kind, msg, detail_msg: detailed }
    }

    pub fn common(msg: Option<&str>, detailed: Option<&str>) -> Self {
        Self::new(SocialErrorKind::Common,msg, detailed)
    }

    pub fn post(msg: Option<&str>, detailed: Option<&str>) -> Self {
        Self::new(SocialErrorKind::Post, msg, detailed)
    }

    pub fn parse_post_url(detail: Option<&str>) -> Self {
        SocialError::post(Some("Error while parse post url"), detail)
    }

    pub fn get_post(detail: Option<&str>) -> Self {
        SocialError::post(Some("Error while get post"), detail)
    }

    pub fn not_implemented(detail: Option<&str>) -> Self {
        SocialError::new(
            SocialErrorKind::NotImplemented,
            Some("Not implemented."),
            detail
        )
    }

    pub fn is_dummy (&self) -> bool {
        if let SocialErrorKind::Dummy = self.kind { return true } else { return false }
    }
    pub fn dummy () -> Self {
        let msg = "Just dummy error here";
        Self::new(SocialErrorKind::Dummy, Some(msg), None)
    }
}

impl fmt::Display for SocialError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SocialError. kind: {:#?}, msg: {}", self.kind, self.msg)
    }
}

impl std::error::Error for SocialError { }
