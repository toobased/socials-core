use log::info;

use crate::{db::SocialsDb, bots::{query::BotQuery, BotCreate, Bot}, social::SocialPlatform};

#[tokio::test]
pub async fn test_crud() {
    env_logger::try_init().ok();
    // test remove tasks
    remove_bots().await;
    // add bot
    add_bot(2).await;
    // find bot
    find_bot().await;
    // get bots
    get_bots().await;
    // get bots time check
    get_bots_time().await;
    // get bots exist fields
    get_bots_fields(false).await;
    // insert bots with fields
    add_bot_fields(1).await;
    // get bots exist fields
    get_bots_fields(true).await;
}

pub async fn remove_bots() {
    let db = SocialsDb::new_test_instance().await.unwrap();
    SocialsDb::delete_many(
        &BotQuery::default(), &db.bots()
    ).await.expect("Some error while deleting");
}

pub async fn add_bot (count: u32) {
    let db = SocialsDb::new_test_instance().await.unwrap();
    for _i in 0..count {
        let bot_create = BotCreate {
            ..Default::default()
        };
        let bot = Bot::create_from(&db, bot_create).await.unwrap();
        SocialsDb::insert_one(
            bot, db.bots()
        ).await.unwrap();
    }
}

pub async fn add_bot_fields (count: u32) {
    let db = SocialsDb::new_test_instance().await.unwrap();
    for _i in 0..count {
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
            bot, db.bots()
        ).await.unwrap();
    }
}

pub async fn find_bot () {
    let db = SocialsDb::new_test_instance().await.unwrap();
    let query = BotQuery::new();
    let _bot = SocialsDb::find_one(
        &query, &db.bots()
    ).await.unwrap();
    // info!("bot is {:#?}", bot);
}

pub async fn get_bots () {
    let db = SocialsDb::new_test_instance().await.unwrap();
    let query = BotQuery::new();
    // query.is_ready();
    let bots = SocialsDb::find(
        &query, &db.bots()
    ).await.unwrap();
    info!("bots are {:#?} {}", bots.items, bots.total);
}

pub async fn get_bots_time () {
    let db = SocialsDb::new_test_instance().await.unwrap();
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

pub async fn get_bots_fields (condition: bool) {
    let db = SocialsDb::new_test_instance().await.unwrap();
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
