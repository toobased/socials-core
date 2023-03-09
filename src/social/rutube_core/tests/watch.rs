use crate::{db::{SocialsDb, DbActions}, tasks::{TaskActionEnum, watch::{WatchAction, WatchTargetData, WatchSettings}, TaskActionType, BotTaskCreate, BotTask}, social::SocialPlatform};
use crate::tests::db_helpers::clean_tasks_db;

static VIDEO_LINK: &'static str = "https://rutube.ru/video/8f365936b26672716ab9b5144cf90495/";
// static INCORRECT_VIDEO_LINK: &'static str = "https://www.youtube.com/watch?v=zuL55W3asdf";

#[tokio::test]
async fn test_watch() {
    env_logger::try_init().ok();
    log::set_max_level(log::LevelFilter::Info);
    let db = SocialsDb::new_test_instance().await.unwrap();

    test_watch_video(&db, VIDEO_LINK, true, false).await;
    // test_watch_video(&db, INCORRECT_VIDEO_LINK, false, true).await;
}

pub async fn test_watch_video (
    db: &SocialsDb,
    link: &str,
    expect_done: bool,
    expect_err: bool
) {
    // clean dbs
    clean_tasks_db(db).await;

    let action = TaskActionEnum::WatchAction(WatchAction {
        data: WatchTargetData {
            watch_count: 2,
            watch_seconds: 2,
            time_spread: 0,
            resource_link: link.to_string(),
            ..Default::default()
        },
        settings: WatchSettings {
            take_screenshot: true,
            ..Default::default()
        },
        ..Default::default()
    });
    let new_task = BotTaskCreate {
        action_type: TaskActionType::Watch,
        platform: SocialPlatform::Rutube,
        action,
        ..Default::default()
    };
    let mut task = BotTask::create_from(&db, new_task).await;
    task.insert_db(db).await.unwrap();
    task.make(&db).await;
    assert_eq!(task.is_done() == expect_done, true, "Task condition done not match");
    assert_eq!(task.is_error() == expect_err, true, "Task condition error not match");
}
