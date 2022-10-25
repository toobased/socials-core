use serde::{Serialize, Deserialize};

use crate::db::DbQuery;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ActionEventQuery {}

impl DbQuery for ActionEventQuery {
}
