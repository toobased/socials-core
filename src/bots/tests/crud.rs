use std::time::{SystemTime, Duration};

use log::info;

use crate::{db::SocialsDb, bots::{query::BotQuery, BotCreate, Bot}, social::SocialPlatform};

#[tokio::test]
pub async fn test_crud() {
    env_logger::try_init().ok();
    let db = SocialsDb::new_test_instance().await.unwrap();
    // test remove tasks
    remove_bots(&db).await;
    // add bot
    add_bot(&db, 4).await;
    // find bot
    find_bot(&db).await;
    // get bots
    get_bots(&db).await;
    // get bots time check
    get_bots_time(&db).await;
    // get bots exist fields
    get_bots_fields(&db, false).await;
    // insert bots with fields
    add_bots_fields(&db, 1).await;
    // get bots exist fields
    get_bots_fields(&db, true).await;
}

pub async fn remove_bots(db: &SocialsDb) {
    SocialsDb::delete_many(
        &BotQuery::default(), &db.bots()
    ).await.expect("Some error while deleting");
}

pub async fn add_bot (db: &SocialsDb, count: u32) {
    for _i in 0..count {
        let rest = Some(SystemTime::now().checked_add(Duration::from_secs(7300)).unwrap());
        let bot_create = BotCreate {
            rest_until: rest,
            ..Default::default()
        };
        let bot = Bot::create_from(&db, bot_create).await.unwrap();
        SocialsDb::insert_one(
            bot, db.bots()
        ).await.unwrap();
    }
}

pub async fn add_bot_fields (db: &SocialsDb) -> Bot {
    let bot_create = BotCreate {
        username: "test".to_string(),
        password: Some("test".to_string()),
        access_token: Some("sometesttokenhere".to_string()),
        platform: SocialPlatform::Vk,
        make_ready: true,
        ..Default::default()
    };
    let bot = Bot::create_from(&db, bot_create).await.unwrap();
    SocialsDb::insert_one(
        bot.clone(), db.bots()
    ).await.unwrap();
    bot
}

pub async fn add_bot_sleep (db: &SocialsDb) -> Bot {
    let rest = SystemTime::now().checked_add(Duration::from_secs(18000)).unwrap();
    let bot_create = BotCreate {
        username: "test".to_string(),
        password: Some("test".to_string()),
        access_token: Some("sometesttokenhere".to_string()),
        platform: SocialPlatform::Vk,
        make_ready: true,
        rest_until: Some(rest),
        ..Default::default()
    };
    let bot = Bot::create_from(&db, bot_create).await.unwrap();
    SocialsDb::insert_one(
        bot.clone(), db.bots()
    ).await.unwrap();
    bot
}

pub async fn add_bots_fields (db: &SocialsDb, count: u32) {
    for _i in 0..count { add_bot_fields(db).await; }
}

pub async fn find_bot (db: &SocialsDb) {
    let query = BotQuery::new();
    let _bot = SocialsDb::find_one(
        &query, &db.bots()
    ).await.unwrap();
    // info!("bot is {:#?}", bot);
}

pub async fn get_bots (db: &SocialsDb) {
    let query = BotQuery::new();
    // query.is_ready();
    let bots = SocialsDb::find(
        &query, &db.bots()
    ).await.unwrap();
    info!("bots are {:#?} {}", bots.items, bots.total);
}

pub async fn get_bots_time (db: &SocialsDb) {
    let mut query = BotQuery::new();
    query.top_old_created();
    let bots = SocialsDb::find(
        &query, &db.bots()
    ).await.unwrap();
    let bot0 = bots.items.get(0).unwrap();
    let bot1 = bots.items.get(1).unwrap();
    let condition = bot0.date_created.lt(&bot1.date_created);
    assert!(condition)
}

pub async fn get_bots_fields (db: &SocialsDb, condition: bool) {
    let mut query = BotQuery::new();
    query.has_token();
    // info!("query is {:#?}", query.collect_filters());
    let bots = SocialsDb::find(
        &query, &db.bots()
    ).await.unwrap();
    // info!("bots are {:#?}", bots.items);
    let check = bots.items.len() > 0;
    assert!(check == condition)
}
