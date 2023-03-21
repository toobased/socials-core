use log::info;

use crate::{tasks::{BotTaskQuery, BotTaskCreate, BotTask, TaskActionType, TaskActionEnum, BotTaskType, like::{LikeAction, LikeTargetData}, ActionExtra, TaskTarget}, db::{SocialsDb, DbFindResult, DbActions, DummyQuery}, social::{post::SocialPost, SocialPlatform}};
use crate::db::DbQuery;

#[tokio::test]
pub async fn test_tasks_crud() {
    env_logger::init();
    // init db
    let db = SocialsDb::new_test_instance().await.unwrap();

    // test remove tasks
    db_remove_tasks(&db).await;
    // test create
    // db_create_task_json().await;
    db_create_task(&db).await;
    // test find many
    db_get_bots_tasks(&db).await;
    // test update one
    db_update_by_id_task(&db).await;
    // test find one
    db_find_one_task(&db).await;
    db_find_one_regular(&db).await;
    db_find_one_browser(&db).await;

    // test create with extra
    test_tasks_with_extra(&db).await;

    // test task validation limits
    test_task_validation_limits(&db).await
}

#[tokio::test]
pub async fn test_tasks_types_crud () {
    let db = SocialsDb::new_test_instance().await.unwrap();
    db_clean_task_types(&db).await;
    db_create_task_type(&db).await;
}

pub async fn db_clean_task_types (db: &SocialsDb) {
    SocialsDb::delete_many(&DummyQuery::default(), &db.task_types()).await.unwrap();
}

pub async fn test_tasks_with_extra (db: &SocialsDb) {
    test_tasks_with_extra_vk(&db).await
}

pub async fn test_tasks_with_extra_vk(db: &SocialsDb) {
    let mut new = BotTaskCreate::default();
    let post = SocialPost::get_post_by_url(
        &SocialPlatform::Vk,
        "https://vk.com/kf_films?w=wall-211982694_1417"
    ).await.unwrap();
    let action = LikeAction {
        extra: ActionExtra {
            post: Some(post),
            ..Default::default()
        },
        ..Default::default()
    };
    new.action = TaskActionEnum::LikeAction(action);
    // new.extra.post = Some(post);
    let mut task = BotTask::create_from(db, new).await;
    task.insert_db(db).await.unwrap();
}

pub async fn test_task_validation_limits (db: &SocialsDb) {
    let mut task = BotTaskCreate {
        platform: SocialPlatform::Vk,
        ..Default::default()
    };
    let action = LikeAction {
        target: TaskTarget::Post,
        data: LikeTargetData {
            like_count: 20,
            ..Default::default()
        },
        ..Default::default()
    };
    task.action = TaskActionEnum::LikeAction(action);
    let is_valid = task.action.validate_limits(&task.platform, db).await;
    info!("is_valid is {:?}", is_valid);
    assert_eq!(is_valid.is_err(), true)
}

pub async fn db_create_task_type(db: &SocialsDb) {
    let traw = r#"{
        "action_type": "Watch",
        "name": "Watch action",
        "targets": [
            {
                "target": "Video",
                "platforms": [
                    { "platform": "Youtube", "count_limit": 500 },
                    { "platform": "Dzen" }
                ]
            }
        ],
        "is_active": true
    }"#;
    let lraw = r#"{
        "action_type": "Like",
        "name": "Like action",
        "targets": [
            {
                "target": "Post",
                "platforms": [
                    { "platform": "Vk", "count_limit": 3 }
                ]
            }
        ],
        "is_active": true
    }"#;
    let target = serde_json::from_str::<BotTaskType>(traw).unwrap();
    SocialsDb::insert_one(target, db.task_types()).await.unwrap();
    let target = serde_json::from_str::<BotTaskType>(lraw).unwrap();
    SocialsDb::insert_one(target, db.task_types()).await.unwrap();
}

pub async fn db_remove_tasks(db: &SocialsDb) {
    SocialsDb::delete_many(
        &BotTaskQuery::default(), &db.bots_tasks()
    ).await.expect("Some error while deleting");
}

pub async fn db_create_task(db: &SocialsDb) {

    let action = crate::tasks::watch::WatchAction {
        data: crate::tasks::watch::WatchTargetData {
            watch_count: 2,
            watch_seconds: 5,
            time_spread: 10,
            ..Default::default()
        },
        ..Default::default()
    };

    let new_task = BotTaskCreate {
        is_active: false,
        title: "testing".to_string(),
        platform: crate::social::SocialPlatform::Youtube,
        is_testing: true,
        // new type
        action_type: TaskActionType::Watch,
        action: TaskActionEnum::WatchAction(action),
        ..Default::default()
    };
    let task: BotTask = BotTask::create_from(&db, new_task).await;
    // println!("task is {:#?}", task);
    SocialsDb::insert_one(task, db.bots_tasks()).await.unwrap();
}

// #[tokio::test]
pub async fn db_create_task_json() {
    let db = SocialsDb::new_test_instance().await.unwrap();
    let task_raw = r#"{
        "is_active": false,
        "title": "testing_stuff",
        "platform": "Vk",
        "is_testing": false,
        "action_type": "Watch",
        "action": {
            "WatchAction": {
                "target": "Video",
                "data": {
                    "watch_count": 5,
                    "watch_seconds": 5,
                    "resource_link": "https://www.youtube.com/watch?v=zuL55W3Ivtk"
                }
            }
        }
    }"#;
    
    let new_task = serde_json::from_str::<BotTaskCreate>(task_raw).unwrap();

    let task: BotTask = BotTask::create_from(&db, new_task).await;
    // println!("task is {:#?}", task);
    SocialsDb::insert_one(task, db.bots_tasks()).await.unwrap();
}

// #[tokio::test]
pub async fn db_get_bots_tasks(db: &SocialsDb) -> DbFindResult<BotTask> {
    let query = BotTaskQuery::default();
    SocialsDb::find(&query, &db.bots_tasks()).await.unwrap()
}

// #[tokio::test]
pub async fn db_update_by_id_task(db: &SocialsDb) {
    let mut find_result = db_get_bots_tasks(&db).await;
    let task = find_result.items.get_mut(0).unwrap();
    task.title = "testing_stuff_new".to_string();
    let _item = SocialsDb::update_by_id(task.id, task.clone(), &db.bots_tasks())
        .await.unwrap();
}

// #[tokio::test]
pub async fn db_find_one_task(db: &SocialsDb) {
    let query = BotTaskQuery {
        title: Some("testing_stuff_new".to_string()),
        is_browser: Some(1),
        ..Default::default()
    };
    info!("query is {:#?}", query.collect_filters());
    let item = SocialsDb::find_one(&query, &db.bots_tasks())
        .await.unwrap().unwrap();
    info!("item is {} {}", item.id, item.title);
    assert_eq!("testing_stuff_new".to_string(), item.title)
}

pub async fn db_find_one_regular(db: &SocialsDb) {
    let item = SocialsDb::find_one(&regular_task_query(), &db.bots_tasks())
        .await.unwrap();
    info!("Found regular {:#?}", item);
}

pub async fn db_find_one_browser(db: &SocialsDb) {
    let item = SocialsDb::find_one(&browser_task_query(), &db.bots_tasks())
        .await.unwrap();
    info!("Found browser {:#?}", item);
}

pub fn regular_task_query() -> BotTaskQuery {
    let mut query = BotTaskQuery::default();
    query
        .is_active()
        .not_browser()
        .top_old_updated();
    query
}

pub fn browser_task_query() -> BotTaskQuery {
    let mut query = BotTaskQuery::default();
    query
        .is_active()
        .is_browser()
        .top_old_updated();
    query
}
