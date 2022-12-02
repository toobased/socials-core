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

pub fn mdb_cond_in(f: &mut Document, key: &str, conditions: Vec<impl Into<bson::Bson>>) {
    let q = doc! { "$in": conditions };
    f.insert(key, q);
}

pub fn mdb_and(f: &mut Document, conditions: Vec<Document>) -> &mut Document {
    f.insert("$and", conditions); f
}

pub fn mdb_or(f: &mut Document, conditions: Vec<Document>) -> &mut Document {
    f.insert("$or", conditions); f
}

pub fn mdb_or_null(f: &mut Document, key: &str, cond: Document) {
    let conditions = vec![cond, doc! { key: { "$eq": bson::Bson::Null } } ];
    mdb_or(f, conditions);
}

pub fn mdb_cond_time(f: &mut Document, key: &str, cond: &str, time: SystemTime, or_null: bool) {
    let k = format!("{}.secs_since_epoch", key);
    let filter = || -> Document {
        let mut r = Document::new();
        r.insert(&k, doc! { cond: time.duration_since(UNIX_EPOCH).unwrap().as_secs_f64() } );
        r
    };
    match or_null {
        true => { mdb_or_null(f, &k, filter()) }
        false => { f.insert(&k, filter()); }
    }
}

pub fn unix_now_secs_f64() -> f64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64()
}
