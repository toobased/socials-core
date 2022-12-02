// use log::info;

use crate::{tasks::{events::{query::ActionEventQuery, ActionEvent}, TaskActionType}, db::{SocialsDb, DbActions}, bots::{query::BotQuery, Bot}, social::SocialPlatform};

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

pub async fn bot_create_event(bot: &Bot, action: TaskActionType, db: &SocialsDb) {
    let mut event = ActionEvent::default();
    // info!("Creating bot {:#?} event", action);
    event
        .set_bot_id(bot.id)
        .set_platform(bot.platform)
        .set_action_type(action)
        .set_amount(1)
        .insert_db(db).await.unwrap();
}

pub async fn check_ready_bots_for(db: &SocialsDb, p: Option<SocialPlatform>, exist: bool) {
    let mut q = BotQuery::default();
    q
        .is_ready_or_awake();
    // info!("q is {}", q.collect_filters());
    q.platform = p;
    let res = SocialsDb::find(&q, &db.bots()).await.unwrap();
    match exist {
        true => assert_ne!(res.items.len(), 0),
        false => assert_eq!(res.items.len(), 0)
    }
}

pub async fn check_ready_bots_for_task_action(db: &SocialsDb, p: &SocialPlatform, a: &TaskActionType, exist: bool) {
    let mut q = BotQuery::default();
    q
        .is_ready_or_awake()
        .with_platform(p.clone())
        .is_awake_for(a.clone());
    // info!("q is {:#?}", q);
    // info!("q is {}", q.collect_filters());
    let res = SocialsDb::find(&q, &db.bots()).await.unwrap();
    match exist {
        true => assert_ne!(res.items.len(), 0),
        false => assert_eq!(res.items.len(), 0)
    }
}
