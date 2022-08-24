use core::fmt;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TaskErrorKind {
    Db,
    IncorrectData,
    ActionError,
    Dummy
}

impl Default for TaskErrorKind {
    fn default () -> Self { Self::Dummy }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct TaskError {
    kind: TaskErrorKind,
    msg: String,
    detail_msg: String
}

impl TaskError {
    pub fn new(kind: TaskErrorKind, msg: Option<String>, detailed: Option<String>) -> Self {
        let msg = msg.unwrap_or("".to_string());
        let detailed = detailed.unwrap_or("".to_string());
        Self { kind, msg, detail_msg: detailed }
    }

    pub fn action_error(msg: Option<String>, detailed: Option<String>) -> Self {
        Self::new(TaskErrorKind::ActionError, msg, detailed)
    }
    pub fn incorrect_link(link: &str) -> Self {
        let mut msg = "Incorrect link: ".to_string();
        msg.push_str(link);
        Self::action_error(Some(msg), None)
    }
    pub fn element_click(info: Option<&str>) -> Self {
        let msg = info.unwrap_or("Cant click on element").to_string();
        Self::action_error(Some(msg), None)
    }
}

impl fmt::Display for TaskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DbError. kind: {:#?}, msg: {}", self.kind, self.msg)
    }
}

impl std::error::Error for TaskError { }
