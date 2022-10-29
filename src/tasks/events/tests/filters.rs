use std::time::{Duration, SystemTime};

// use log::info;
use crate::{tasks::events::query::ActionEventQuery, db::SocialsDb};

#[tokio::test]
pub async fn test_date_filter () {
    // logging
    env_logger::try_init().ok();
    log::set_max_level(log::LevelFilter::Info);

    let db = SocialsDb::new_test_instance().await.unwrap();

    // clear db
    clear_events_db(&db).await;

    super::crud::db_insert_dummy_event(&db).await;

    test_greater(&db).await;
    test_less(&db).await;

    test_hours(&db).await;
}

pub async fn clear_events_db(db: &SocialsDb) {
    let q = ActionEventQuery::default();
    SocialsDb::delete_many(&q, &db.action_events())
        .await.unwrap();
}

pub async fn test_greater(db: &SocialsDb) {
    let duration = Duration::from_secs(2);
    let now = SystemTime::now();
    let mut query = ActionEventQuery::default();
    let diff = now.checked_add(duration).unwrap();
    query.with_date_created_gte(diff);
    let search = SocialsDb::find(&query, &db.action_events()).await.unwrap();
    assert_eq!(0, search.items.len());

    let now = SystemTime::now();
    let diff = now.checked_sub(duration).unwrap();
    query.with_date_created_gte(diff);
    let search = SocialsDb::find(&query, &db.action_events()).await.unwrap();
    assert_eq!(1, search.items.len());
}

pub async fn test_less(db: &SocialsDb) {
    let duration = Duration::from_secs(2);
    let now = SystemTime::now();
    let mut query = ActionEventQuery::default();
    let diff = now.checked_add(duration).unwrap();
    query.with_date_created_lte(diff);
    let search = SocialsDb::find(&query, &db.action_events()).await.unwrap();
    assert_eq!(1, search.items.len());

    let now = SystemTime::now();
    let diff = now.checked_sub(duration).unwrap();
    query.with_date_created_lte(diff);
    let search = SocialsDb::find(&query, &db.action_events()).await.unwrap();
    assert_eq!(0, search.items.len());
}

pub async fn test_hours (db: &SocialsDb) {
    let mut query = ActionEventQuery::default();
    query.with_last_1hr();
    let search = SocialsDb::find(&query, &db.action_events()).await.unwrap();
    assert_eq!(1, search.items.len());
    query.with_last_24hr();
    let search = SocialsDb::find(&query, &db.action_events()).await.unwrap();
    assert_eq!(1, search.items.len());
}
