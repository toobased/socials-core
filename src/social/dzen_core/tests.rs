use crate::{db::SocialsDb, tasks::{TaskActionEnum, watch::{WatchAction, WatchTargetData}, TaskActionType, BotTaskCreate, BotTask}, social::SocialPlatform};

#[tokio::test]
async fn test_task_actions () {
    env_logger::init();
    test_watch_video().await;
}

pub async fn test_watch_video () {
    let db = SocialsDb::new_test_instance().await.unwrap();
    let action = TaskActionEnum::WatchAction(WatchAction {
        data: WatchTargetData {
            watch_count: 2,
            watch_seconds: 5,
            time_spread: 0,
            resource_link: "https://zen.yandex.ru/video/watch/6308b747f90f894d66453ba7".to_string(),
            ..Default::default()
        },
        ..Default::default()
    });
    let new_task = BotTaskCreate {
        action_type: TaskActionType::Watch,
        platform: SocialPlatform::Dzen,
        action,
        ..Default::default()
    };
    let mut task = BotTask::create_from(&db, new_task).await;
    task.make(&db).await;
}
