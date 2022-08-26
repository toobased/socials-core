use log::info;

use crate::{tasks::{BotTaskQuery, BotTaskCreate, BotTask, TaskActionType, TaskActionEnum}, db::{SocialsDb, DbFindResult}};
use crate::db::DbQuery;

#[tokio::test]
pub async fn test_crud() {
    env_logger::init();
    // test remove tasks
    db_remove_tasks().await;
    // test create
    // db_create_task_json().await;
    db_create_task().await;
    // test find many
    db_get_bots_tasks().await;
    // test update one
    db_update_by_id_task().await;
    // test find one
    db_find_one_task().await;
}

pub async fn db_remove_tasks() {
    let db = SocialsDb::new_test_instance().await.unwrap();
    SocialsDb::delete_many(
        &BotTaskQuery::default(), &db.bots_tasks()
    ).await.expect("Some error while deleting");
}

pub async fn db_create_task() {
    let db = SocialsDb::new_test_instance().await.unwrap();

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
pub async fn db_get_bots_tasks() -> DbFindResult<BotTask> {
    let db = SocialsDb::new_test_instance().await.unwrap();
    let query = BotTaskQuery::default();
    SocialsDb::find(&query, &db.bots_tasks()).await.unwrap()
}

// #[tokio::test]
pub async fn db_update_by_id_task() {
    let db = SocialsDb::new_test_instance().await.unwrap();
    let mut find_result = db_get_bots_tasks().await;
    let task = find_result.items.get_mut(0).unwrap();
    task.title = "testing_stuff_new".to_string();
    let _item = SocialsDb::update_by_id(task.id, task, &db.bots_tasks())
        .await.unwrap();
}

// #[tokio::test]
pub async fn db_find_one_task() {
    let db = SocialsDb::new_test_instance().await.unwrap();
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
