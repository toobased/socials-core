use std::time::{UNIX_EPOCH, SystemTime, Duration };

use bson::{Document, doc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Timestamp(u64);

impl Timestamp {
    pub fn now () -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Timestamp(Timestamp::now())
    }
}

pub fn pretty_duration(d: Duration) -> String {
    let s = d.as_secs();
    let hrs: u64 = s.div_euclid(60).div_euclid(60);
    let mins = (s - (hrs * 60 * 60)).div_euclid(60);
    let sec = s - ((hrs * 60 * 60) + (mins * 60));
    format!("{} hrs {} min {} secs ", hrs, mins, sec)
}

pub fn mdb_cond_or_null(f: &mut Document, key: &str, cond: Document) {
    let mut c1 = Document::new(); c1.insert(key, cond);
    let mut c2 = Document::new(); c2.insert(key, bson::Bson::Null);
    f.insert("$or", vec![c1,c2]);
}

pub fn unix_now_secs_f64() -> f64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64()
}
