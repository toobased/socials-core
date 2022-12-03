use log::info;

use crate::{tasks::{BotTaskQuery, TaskTarget, BotTaskCreate, TaskActionType, TaskActionEnum, BotTask}, bots::{query::BotQuery, BotCreate, Bot}, db::{SocialsDb, DbActions}, social::SocialPlatform};

static LB: &str = "DbHelper";

pub async fn clean_tasks_db (db: &SocialsDb) {
    let q = BotTaskQuery::new();
    SocialsDb::delete_many(&q, &db.bots_tasks()).await.unwrap();
}

pub async fn clean_bots_db (db: &SocialsDb) {
    let q = BotQuery::new();
    SocialsDb::delete_many(&q, &db.bots()).await.unwrap();
}

pub async fn clean_events_db (db: &SocialsDb) {
    let q = BotQuery::new();
    SocialsDb::delete_many(&q, &db.action_events()).await.unwrap();
}

pub async fn insert_like_task(
    db: &SocialsDb,
    item_id: &str, owner_id: &str, resource_link: &str, platform: &SocialPlatform,
    count: u64
) {
    info!("[{}] invoke `insert_like_task` for {:#?}", LB, platform);
    let action = crate::tasks::like::LikeAction {
        target: TaskTarget::Post,
        data: crate::tasks::like::LikeTargetData {
            like_count: count,
            item_id: Some(item_id.to_string()),
            owner_id: Some(owner_id.to_string()),
            resource_link: Some(resource_link.to_string()),
            ..Default::default()
        },
        ..Default::default()
    };

    let new_task = BotTaskCreate {
        is_active: true,
        title: "testing".to_string(),
        platform: platform.clone(),
        is_testing: true,
        // new type
        action_type: TaskActionType::Like,
        action: TaskActionEnum::LikeAction(action),
        ..Default::default()
    };
    let task: BotTask = BotTask::create_from(&db, new_task).await;
    // println!("task is {:#?}", task);
    SocialsDb::insert_one(task, db.bots_tasks()).await.unwrap();
}

pub async fn insert_test_bot(db: &SocialsDb, token: &str, platform: &SocialPlatform) {
    info!("`[{}] insert_test_bot` for {:#?} with token {}", LB, platform, token);
    let new_bot = BotCreate {
        access_token: Some(token.to_string()),
        platform: platform.clone(),
        make_ready: true,
        ..Default::default()
    };
    let mut bot = Bot::create_from(&db, new_bot).await.unwrap();
    bot.insert_db(&db).await.unwrap();
}
