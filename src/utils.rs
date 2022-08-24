use std::time::{UNIX_EPOCH, SystemTime};

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Timestamp(u64);

impl Timestamp {
    pub fn now () -> u64 {
        // FIXME can fall in prod?
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Timestamp(Timestamp::now())
        /*
        Timestamp(
            SystemTime::now().duration_since(UNIX_EPOCH)
                .expect("Time goes backwards?").as_secs()
        )
        */
    }
}
