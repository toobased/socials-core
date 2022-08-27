use std::fmt;

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum DbErrorKind {
    NoDbConnectionString,
    NoDbName,
    InvalidConnectionString,
    ConnectionError,
    ErrorWhileFind,
    CursorCollectError,
    InsertError,
    UpdateError,
    ReplaceError,
    DeleteError,
    NotFound
}

// TODO make serializable
#[derive(Clone, Debug, Serialize)]
pub struct DbError {
    pub kind: DbErrorKind,
    pub msg: String,
}

impl DbError {
    pub fn new(kind: DbErrorKind, msg: Option<String>) -> Self {
        let msg = match msg {
            Some(m) => m,
            None => "no message".to_string()
        };
        Self {
            kind,
            msg
        }
    }

    pub fn cursor_collect() -> Self {
        let msg = "Error occurred while cursor collect".to_string();
        Self::new(DbErrorKind::CursorCollectError, Some(msg))
    }

    pub fn error_while_find() -> Self {
        let msg = "Error occurred while find in db".to_string();
        Self::new(DbErrorKind::ErrorWhileFind, Some(msg))
    }

    pub fn not_found(msg: Option<String>) -> Self {
        Self::new(DbErrorKind::NotFound, msg)
    }

    pub fn insert_error(msg: Option<String>) -> Self {
        Self::new(DbErrorKind::InsertError, msg)
    }

    pub fn replace_error(msg: Option<String>) -> Self {
        Self::new(DbErrorKind::ReplaceError, msg)
    }

    pub fn delete_error() -> Self {
        let msg = "Error while deleting items".to_string();
        Self::new(DbErrorKind::DeleteError, Some(msg))
    }

    pub fn db_connection_string() -> Self {
        let msg = "No db connection string specified in env".to_string();
        Self::new(DbErrorKind::NoDbConnectionString, Some(msg))
    }

    pub fn db_name() -> Self {
        let msg = "No db name specified in env".to_string();
        Self::new(DbErrorKind::NoDbName, Some(msg))
    }

    pub fn invalid_connection_string() -> Self {
        Self {
            kind: DbErrorKind::InvalidConnectionString,
            msg: "Cant parse connection string. Check it".to_string()
        }
    }

    pub fn connection_error() -> Self {
        Self {
            kind: DbErrorKind::ConnectionError,
            msg: "Cant connect to db. Check its availability & connection string".to_string()
        }
    }
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DbError. kind: {:#?}, msg: {}", self.kind, self.msg)
    }
}

impl std::error::Error for DbError { }
