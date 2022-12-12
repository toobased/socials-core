use crate::{db::{SocialsDb, DbActions}, tasks::{TaskActionEnum, watch::{WatchAction, WatchTargetData}, TaskActionType, BotTaskCreate, BotTask}, social::SocialPlatform};
use crate::tests::db_helpers::clean_tasks_db;

static VIDEO_LINK: &'static str = "https://www.youtube.com/watch?v=zuL55W3Ivtk";

#[tokio::test]
async fn test_watch() {
    env_logger::try_init().ok();
    log::set_max_level(log::LevelFilter::Info);
    let db = SocialsDb::new_test_instance().await.unwrap();

    test_watch_video(&db).await;
}

pub async fn test_watch_video (db: &SocialsDb) {
    // clean dbs
    clean_tasks_db(db).await;

    let action = TaskActionEnum::WatchAction(WatchAction {
        data: WatchTargetData {
            watch_count: 2,
            watch_seconds: 10,
            time_spread: 0,
            resource_link: VIDEO_LINK.to_string(),
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
    task.insert_db(db).await.unwrap();
    task.make(&db).await;
}
