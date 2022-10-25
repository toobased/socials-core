// use log::info;

use crate::{tasks::events::ActionEvent, db::{SocialsDb, DbActions}};

#[tokio::test]
pub async fn test_events_crud() {
    env_logger::try_init().ok();
    let db = SocialsDb::new_test_instance().await.unwrap();
    db_insert_dummy_event(&db).await;
}

pub async fn db_insert_dummy_event(db: &SocialsDb) { 
    let mut event = make_dummy_event();
    event.insert_db(&db).await.unwrap();
    // event.delete_db(&db).await.unwrap();
}
pub fn make_dummy_event() -> ActionEvent { ActionEvent::default() }


