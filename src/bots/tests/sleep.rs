use std::time::{SystemTime, Duration};

use log::info;

use crate::{db::{SocialsDb, DbActions}, bots::{query::BotQuery, BotCreate, Bot}, tasks::{events::{ActionEvent, query::ActionEventQuery}, TaskActionType, like::LikeAction}, social::SocialPlatform};

#[tokio::test]
pub async fn test_bot_sleep () {

    // logging
    env_logger::try_init().ok();
    log::set_max_level(log::LevelFilter::Info);

    // db
    let db = SocialsDb::new_test_instance().await.unwrap();

    // clear dbs
    clear_events_db(&db).await;
    clear_bots_db(&db).await;

    let mut bot = create_bot(&db).await;

    for _i in 0..18 {
        bot_create_event(&bot, TaskActionType::Like, &db).await;
    }

    let like_action = LikeAction::default();

    info!("--- Testing get bots real sleep time ---");
    bot.after_action_sleep(&like_action, &db).await;

    bot.update_db(&db).await.unwrap();

    get_bots_for_task(&db).await;

    info!("--- Testing get bots with ended sleep time ---");
    let late = SystemTime::now().checked_sub(Duration::from_secs(2)).unwrap();
    bot.actions_rest.like = Some(late);
    bot.update_db(&db).await.unwrap();
    get_bots_for_task(&db).await;

    info!("--- Testing get bots with NONE sleep time ---");
    bot.actions_rest.like = None;
    bot.update_db(&db).await.unwrap();
    get_bots_for_task(&db).await;
}

pub async fn clear_events_db(db: &SocialsDb) {
    let q = ActionEventQuery::default();
    SocialsDb::delete_many(&q, &db.action_events())
        .await.unwrap();
}

pub async fn clear_bots_db(db: &SocialsDb) {
    let q = BotQuery::default();
    SocialsDb::delete_many(&q, &db.bots())
        .await.unwrap();
}

pub async fn create_bot (db: &SocialsDb) -> Bot {
    let new_bot = BotCreate {
        make_ready: true,
        platform: SocialPlatform::Vk,
        ..Default::default()
    };
    let mut bot = Bot::create_from(db, new_bot).await.unwrap();
    bot.insert_db(db).await.unwrap();
    bot
}

pub async fn bot_create_event(bot: &Bot, action: TaskActionType, db: &SocialsDb) {
    let mut event = ActionEvent::default();
    info!("Creating bot {:#?} event", action);
    event
        .set_bot_id(bot.id)
        .set_platform(bot.platform)
        .set_action_type(action)
        .set_amount(1)
        .insert_db(db).await.unwrap();
}

pub async fn get_bots_for_task(db: &SocialsDb) {
    let mut q = BotQuery::default();
    q
        .is_ready()
        .with_platform(SocialPlatform::Vk);
    let res = SocialsDb::find(&q, &db.bots()).await.unwrap();
    info!("[NO_SLEEP] Found bots length: {}", res.items.len());

    let mut q = BotQuery::default();
    q
        .is_ready()
        .is_awake_for(TaskActionType::Like)
        .with_platform(SocialPlatform::Vk);
    let res = SocialsDb::find(&q, &db.bots()).await.unwrap();
    info!("[SLEEP] Found bots length: {}", res.items.len());
}
