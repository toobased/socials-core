use std::time::SystemTime;

use log::info;

use crate::{db::SocialsDb, tests::db_helpers::{clean_tasks_db, insert_test_bot, insert_like_task}, social::SocialPlatform, tasks::BotTaskQuery, bots::query::BotQuery};

use super::db_helpers::clear_bots_db;

static ITEM_ID: &str = "";
static OWNER_ID: &str = "";
static RESOURCE_LINK_SUCESS: &str = "https://vk.com/kf_films?w=wall-211982694_1790";
static PLATFORM: SocialPlatform = SocialPlatform::Vk;

#[tokio::test]
pub async fn test_metrics_all () {
    env_logger::try_init().ok();
    log::set_max_level(log::LevelFilter::Info);
    let db = SocialsDb::new_test_instance().await.unwrap();

    test_common_metrics(&db).await;
}

pub async fn test_common_metrics(db: &SocialsDb) {
    // clean dbs
    clear_bots_db(&db).await;
    clean_tasks_db(&db).await;
    // insert test bot
    for _i in 0..1 { insert_test_bot(db, "testtoken", &SocialPlatform::Vk).await; }
    // insert test task
    for _i in 0..1 { insert_like_task(&db, ITEM_ID, OWNER_ID, RESOURCE_LINK_SUCESS, &PLATFORM, 18, 0).await; }

    // remember metrics
    let q = BotQuery::default();
    let bot = SocialsDb::find_one(&q, &db.bots()).await.unwrap().unwrap();
    let old_time = bot.date_created;
    // make task
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
                t.next_run_time = Some(SystemTime::now()); // ⚠️ forced to just make it run
                t.make(&db).await;
            }
        };
    }
    // check metrics
    let q = BotQuery::default();
    let bot = SocialsDb::find_one(&q, &db.bots()).await.unwrap().unwrap();

    let date_updated = bot.date_updated;
    let last_used = bot.last_used;
    let rest_like = bot.actions_rest.like;

    assert_eq!(date_updated > old_time, true, "Date updated was not updated | not set? 🤔");

    assert_eq!(last_used.is_some(), true, "Last used was not set? ⚠️");
    assert_eq!(last_used.unwrap() > old_time, true, "Last used was not updated correctly, its stale 🤔");

    assert_eq!(rest_like.is_some(), true, "Rest for like was not set? ⚠️");
    assert_eq!(rest_like.unwrap() > old_time, true, "Rest for like was not updated correctly, its stale 🤔");

}
