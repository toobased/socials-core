use core::fmt;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BotErrorKind { Dummy, Common, Auth, Access, Ban }

impl Default for BotErrorKind { fn default () -> Self { Self::Dummy } }

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BotError {
    kind: BotErrorKind,
    msg: String,
    detail_msg: String,
}

impl BotError {
    pub fn new(kind: BotErrorKind, msg: Option<String>, detailed: Option<String>) -> Self {
        let msg = msg.unwrap_or("".to_string());
        let detailed = detailed.unwrap_or("".to_string());
        Self { kind, msg, detail_msg: detailed }
    }

    pub fn common(msg: Option<String>, detailed: Option<String>) -> Self {
        Self::new(BotErrorKind::Common,msg, detailed)
    }

    pub fn is_dummy (&self) -> bool {
        if let BotErrorKind::Dummy = self.kind { return true } else { return false }
    }
    pub fn dummy () -> Self {
        let msg = "Just dummy error here".to_string();
        Self::new(BotErrorKind::Dummy, Some(msg), None)
    }

    pub fn auth (msg: Option<String>, detailed: Option<String>) -> Self {
        Self::new(BotErrorKind::Auth, msg, detailed)
    }
    pub fn captcha (msg: Option<String>, detailed: Option<String>) -> Self {
        Self::new(BotErrorKind::Access, msg, detailed)
    }
    pub fn access_denied (msg: Option<String>, detailed: Option<String>) -> Self {
        Self::new(BotErrorKind::Access, msg, detailed)
    }
    pub fn ban (msg: Option<String>, detailed: Option<String>) -> Self {
        Self::new(BotErrorKind::Ban, msg, detailed)
    }
}

impl fmt::Display for BotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DbError. kind: {:#?}, msg: {}", self.kind, self.msg)
    }
}

impl std::error::Error for BotError { }
