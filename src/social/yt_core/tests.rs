use crate::{db::SocialsDb, tasks::{TaskActionEnum, watch::{WatchAction, WatchTargetData}, TaskActionType, BotTaskCreate, BotTask}, social::SocialPlatform};

#[tokio::test]
async fn test_task_actions () {
    test_watch_video().await;
}

pub async fn test_watch_video () {
    // env_logger::init();
    let db = SocialsDb::new_test_instance().await.unwrap();
    let action = TaskActionEnum::WatchAction(WatchAction {
        data: WatchTargetData {
            watch_count: 2,
            watch_seconds: 5,
            time_spread: 0, // 3600 - 60 minutes for task
            resource_link: "https://www.youtube.com/watch?v=zuL55W3Ivtk&t=3s".to_string(),
// https://zen.yandex.ru/video/watch/6308b747f90f894d66453ba7
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
