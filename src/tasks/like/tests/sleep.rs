use std::time::{SystemTime, Duration};

use log::info;

use crate::{db::SocialsDb, tests::db_helpers::{clean_tasks_db, clean_bots_db, clean_events_db, insert_like_task, insert_test_bot}, social::SocialPlatform, tasks::BotTaskQuery};

static ITEM_ID: &str = "";
static OWNER_ID: &str = "";
static RESOURCE_LINK_SUCESS: &str = "https://vk.com/kf_films?w=wall-211982694_1790";
// static RESOURCE_LINK_FAIL: &str = "asdfasdf";
// https://vk.com/kf_films?w=wall-211982694_1790
static PLATFORM: SocialPlatform = SocialPlatform::Vk;

#[tokio::test]
pub async fn test_sleep () {
    let db = SocialsDb::new_test_instance().await.unwrap();
    // logging
    env_logger::try_init().ok();
    log::set_max_level(log::LevelFilter::Info);
    test_task_sleep(&db, 1 * 60).await;
}

pub async fn test_task_sleep (db: &SocialsDb, time_spread: u64) {
    // clean dbs
    clean_tasks_db(&db).await;
    clean_bots_db(&db).await;
    clean_events_db(&db).await;
    // insert tasks
    for _i in 0..1 { insert_like_task(&db, ITEM_ID, OWNER_ID, RESOURCE_LINK_SUCESS, &PLATFORM, 18, time_spread).await; }
    // insert bots
    for _i in 0..10 { insert_test_bot(db, "testtoken", &SocialPlatform::Vk).await; }
    // making task
    let mut old_sleep_until = 0;
    let old_now = SystemTime::now();
    for _i in 0..1 {
        let mut q = BotTaskQuery::new();
        q
            .not_sleep()
            .not_browser()
            .is_active()
            .top_old_updated();
        let task = SocialsDb::find_one(&q, &db.bots_tasks())
            .await.unwrap();
        match task {
            None => info!("No task to do"),
            Some(mut t) => {
                t.make(&db).await;
                old_sleep_until = t.next_run_time.unwrap().duration_since(old_now).unwrap().as_secs();
            }
        };
    }
    tokio::time::sleep(Duration::from_secs(3)).await;
    // making task
    for _i in 0..1 {
        let mut q = BotTaskQuery::new();
        q
            .not_sleep()
            .not_browser()
            .is_active()
            .top_old_updated();
        let task = SocialsDb::find_one(&q, &db.bots_tasks())
            .await.unwrap();
        match task {
            None => info!("No task to do"),
            Some(mut t) => { t.make(&db).await }
        };
    }
    let q = BotTaskQuery::new();
    let task = SocialsDb::find_one(&q, &db.bots_tasks()).await.unwrap().unwrap();
    let fresh_sleep_until = task.next_run_time.unwrap().duration_since(old_now).unwrap().as_secs();
    assert_eq!(fresh_sleep_until > old_sleep_until, true, "task is not setting sleep after interaction?");
}
