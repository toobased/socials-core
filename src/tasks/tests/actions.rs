use crate::{tasks::{BotTask, BotTaskCreate, TaskActionType, TaskActionEnum, watch::{WatchAction, WatchTargetData}, BotTaskQuery}, db::SocialsDb, social::SocialPlatform};

#[tokio::test]
async fn test_task_actions () {
    // test_task_like().await;
    test_task_watch().await;
    db_remove_tasks().await;
    // est_task_watch_db().await;
}

pub async fn db_remove_tasks() {
    let db = SocialsDb::new_test_instance().await.unwrap();
    SocialsDb::delete_many(
        &BotTaskQuery::default(), &db.bots_tasks()
    ).await.expect("Some error while deleting");
}

pub async fn test_task_watch () {
    env_logger::init();
    let db = SocialsDb::new_test_instance().await.unwrap();
    let action = TaskActionEnum::WatchAction(WatchAction {
        data: WatchTargetData {
            watch_count: 0,
            watch_seconds: 5,
            time_spread: 0, // 3600 - 60 minutes for task
            resource_link: "https://www.youtube.com/watch?v=zuL55W3Ivtk&t=3s".to_string(),
            ..Default::default()
        },
        ..Default::default()
    });
    let new_task = BotTaskCreate {
        action_type: TaskActionType::Watch,
        platform: SocialPlatform::Youtube,
        action,
        ..Default::default()
    };
    let mut task = BotTask::create_from(&db, new_task).await;
    task.make().await;
}

pub async fn test_task_watch_db () {
    env_logger::init();
    let db = SocialsDb::new_test_instance()
        .await.unwrap();
    let action = TaskActionEnum::WatchAction(WatchAction {
        data: WatchTargetData {
            watch_count: 2,
            watch_seconds: 5,
            time_spread: 0, // 3600 - 60 minutes for task
            resource_link: "https://www.youtube.com/watch?v=zuL55W3Ivtk&t=3s".to_string(),
            ..Default::default()
        },
        ..Default::default()
    });
    let new_task = BotTaskCreate {
        action_type: TaskActionType::Watch,
        platform: SocialPlatform::Youtube,
        action,
        ..Default::default()
    };
    let mut task = BotTask::create_from(&db, new_task).await;
    SocialsDb::insert_one(task.clone(), db.bots_tasks()).await.expect(
        "Cant insert task"
    );
    task.make().await;
    task.update_db(&db).await.expect("Cant update task in db");
}
