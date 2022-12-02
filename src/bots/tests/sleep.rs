use std::time::{SystemTime, Duration};

use log::info;

use crate::{db::SocialsDb, bots::tests::{db_helpers::{clear_events_db, clear_bots_db, bot_create_event, check_ready_bots_for_task_action, check_ready_bots_for}, crud::add_bot_fields}, tasks::{TaskActionType, like::LikeAction, TaskAction}, social::SocialPlatform};

use super::crud::add_bot_sleep;

#[tokio::test]
pub async fn test_bot_sleep () {
    // logging
    env_logger::try_init().ok();
    log::set_max_level(log::LevelFilter::Info);
    // db
    let db = SocialsDb::new_test_instance().await.unwrap();
    // test bot global sleep
    test_bot_global_sleep(&db).await;
    // test sleep Vk like actions
    test_bot_action_sleep(&db, SocialPlatform::Vk, TaskActionType::Like, LikeAction::default()).await;
}

async fn test_bot_global_sleep(db: &SocialsDb) {
    info!("--- [Bot global sleep] init test ---");
    // clear dbs
    clear_bots_db(&db).await;
    let mut bot = add_bot_sleep(&db).await;
    assert_ne!(bot.rest_until, None);

    info!("--- [Bot global sleep] test no bots when they sleep ---");
    check_ready_bots_for(&db, None, false).await;

    info!("--- [Bot global sleep] test has bots when sleep elapsed ---");
    let elapsed_sleep = SystemTime::now().checked_sub(Duration::from_secs(10)).unwrap();
    bot
        .set_sleep_until(elapsed_sleep)
        .update_db(&db).await.unwrap();
    check_ready_bots_for(&db, None, true).await;

    info!("--- [Bot global sleep] test has bots when no sleep specified (status READY) ---");
    bot
        .clear_sleep_until()
        .set_status_ready()
        .update_db(&db).await.unwrap();
    check_ready_bots_for(&db, None, true).await;

    info!("--- [Bot global sleep] test has bots when no sleep specified (status RESTING) ---");
    bot
        .clear_sleep_until()
        .set_status_resting()
        .update_db(&db).await.unwrap();
    check_ready_bots_for(&db, None, true).await;
}

async fn test_bot_action_sleep(db: &SocialsDb, p: SocialPlatform, a: TaskActionType, ad: impl TaskAction) {
    // clear dbs
    clear_events_db(&db).await;
    clear_bots_db(&db).await;

    let mut bot = add_bot_fields(&db).await;

    for _i in 0..18 { bot_create_event(&bot, a.clone(), &db).await; }

    info!("--- [Bot action sleep] init test for {:#?} {:#?} ---", &p, &a);

    info!("--- [Bot action sleep] test no bots, should sleep after action ---");
    bot.after_action_sleep(&ad, &db).await;
    bot.update_db(&db).await.unwrap();
    check_ready_bots_for_task_action(&db, &p, &a, false).await;

    info!("--- [Bot action sleep] test has bots, sleep elapsed ---");
    let late = SystemTime::now().checked_sub(Duration::from_secs(2)).unwrap();
    bot.actions_rest.like = Some(late);
    bot.update_db(&db).await.unwrap();
    check_ready_bots_for_task_action(&db, &p, &a, true).await;

    info!("--- [Bot action sleep] test has bots, action sleep is NONE --");
    bot.actions_rest.like = None;
    bot.update_db(&db).await.unwrap();
    check_ready_bots_for_task_action(&db, &p, &a, true).await;

    // clear dbs
    clear_events_db(&db).await;
    clear_bots_db(&db).await;
}
