use core::fmt;

use serde::{Serialize, Deserialize};

use crate::{social::errors::SocialError, db::errors::DbError};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TaskErrorKind {
    Db,
    IncorrectData,
    Limits,
    ActionError,
    Dummy,
    NotImplemented,
    Unknown,
    SocialError
}

impl Default for TaskErrorKind {
    fn default () -> Self { Self::Dummy }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct TaskError {
    pub kind: TaskErrorKind,
    pub msg: String,
    pub detail_msg: String
}

impl TaskError {
    pub fn new(kind: TaskErrorKind, msg: Option<&str>, detailed: Option<&str>) -> Self {
        let msg = msg.unwrap_or("").to_string();
        let detailed = detailed.unwrap_or("").to_string();
        Self { kind, msg, detail_msg: detailed }
    }

    pub fn action_error(msg: Option<&str>, detailed: Option<&str>) -> Self {
        Self::new(TaskErrorKind::ActionError, msg, detailed)
    }
    pub fn incorrect_link(link: &str) -> Self {
        let mut msg = "Incorrect link: ".to_string();
        msg.push_str(link);
        Self::action_error(Some(&msg), None)
    }
    pub fn element_click(info: Option<&str>) -> Self {
        let msg = info.unwrap_or("Cant click on element").to_string();
        Self::action_error(Some(&msg), None)
    }
    pub fn task_not_implemented() -> Self {
        let msg = "Task action not implemented yet".to_string();
        Self::action_error(Some(&msg), None)
    }

    pub fn invalid_data(detail: Option<&str>) -> Self {
        Self::new(TaskErrorKind::IncorrectData, Some("Invalid action data"), detail)
    }

    pub fn invalid_count_limit(detail: Option<&str>) -> Self {
        Self::new(TaskErrorKind::Limits, Some("Reach task limits"), detail)
    }

    pub fn social_error(msg: Option<&str>, detail: Option<&str>) -> Self {
        Self::new(TaskErrorKind::IncorrectData, msg, detail)
    }

    pub fn db_error(msg: Option<&str>, detail: Option<&str>) -> Self {
        Self::new(TaskErrorKind::Db, msg, detail)
    }

    pub fn dummy () -> Self {
        let msg = "Just dummy error here";
        Self::action_error(Some(&msg), None)
    }

    pub fn unknown (msg: Option<&str>, detailed: Option<&str>) -> Self {
        Self::new(TaskErrorKind::Unknown, msg, detailed)
    }

    pub fn with_machine_info (&mut self) -> &mut Self {
        let hst = gethostname::gethostname();
        self.detail_msg.push_str(hst.to_str().unwrap_or("unknown"));
        self
    }
}

impl fmt::Display for TaskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DbError. kind: {:#?}, msg: {}", self.kind, self.msg)
    }
}

impl std::error::Error for TaskError { }

impl From<SocialError> for TaskError {
    fn from(v: SocialError) -> Self {
        Self::social_error(Some(&v.msg), Some(&v.detail_msg))
    }
}

impl From<DbError> for TaskError {
    fn from(v: DbError) -> Self {
        Self::db_error(Some(&v.msg), Some(&v.detail))
    }
}

