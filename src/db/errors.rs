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
    pub detail: String
}

impl DbError {
    pub fn new(kind: DbErrorKind, msg: Option<&str>, detail: Option<&str>) -> Self {
        let msg = msg.unwrap_or("no message").to_string();
        let detail = detail.unwrap_or("no message").to_string();
        Self { kind, msg, detail }
    }

    pub fn cursor_collect() -> Self {
        let msg = "Error occurred while cursor collect";
        Self::new(DbErrorKind::CursorCollectError, Some(msg), None)
    }

    pub fn error_while_find() -> Self {
        let msg = "Error occurred while find in db";
        Self::new(DbErrorKind::ErrorWhileFind, Some(msg), None)
    }

    pub fn not_found(msg: Option<&str>) -> Self {
        Self::new(DbErrorKind::NotFound, msg, None)
    }

    pub fn insert_error(msg: Option<&str>) -> Self {
        Self::new(DbErrorKind::InsertError, msg, None)
    }

    pub fn replace_error(msg: Option<&str>) -> Self {
        Self::new(DbErrorKind::ReplaceError, msg, None)
    }

    pub fn delete_error() -> Self {
        let msg = "Error while deleting items";
        Self::new(DbErrorKind::DeleteError, Some(msg), None)
    }

    pub fn db_connection_string() -> Self {
        let msg = "No db connection string specified in env";
        Self::new(DbErrorKind::NoDbConnectionString, Some(msg), None)
    }

    pub fn db_name() -> Self {
        let msg = "No db name specified in env";
        Self::new(DbErrorKind::NoDbName, Some(msg), None)
    }

    pub fn invalid_connection_string() -> Self {
        Self::new(
            DbErrorKind::InvalidConnectionString,
            Some("Cant parse connection string. Check it"),
            None
        )
    }

    pub fn connection_error() -> Self {
        Self::new(
            DbErrorKind::ConnectionError,
            Some("Cant connect to db. Check its availability & connection string"),
            None
        )
    }
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DbError. kind: {:#?}, msg: {}", self.kind, self.msg)
    }
}

impl std::error::Error for DbError { }
