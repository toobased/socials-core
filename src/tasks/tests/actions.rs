// use log::debug;

use log::debug;

use crate::{tasks::{BotTask, BotTaskCreate, TaskActionType, TaskActionEnum, watch::{WatchAction, WatchTargetData}, BotTaskQuery}, db::{SocialsDb, DbActions}, social::SocialPlatform};

pub mod yt {

    #[tokio::test]
    pub async fn test_watch_yt () {
        env_logger::try_init().ok();
        log::set_max_level(log::LevelFilter::Info);
        let db = super::SocialsDb::new_test_instance().await.unwrap();
        super::db_remove_tasks(&db).await;
        for _i in 0..3 { super::test_task_watch(&db).await; }
    }
}

#[tokio::test]
pub async fn test_task_actions () {
    env_logger::try_init().ok();
    let _db = SocialsDb::new_test_instance().await.unwrap();
    // test_task_like().await;
    // for _i in 0..4 { test_task_watch(&db).await; }
    // db_remove_tasks(&db).await;
    // test_task_watch_db().await;
    // create_test_watch_actions(2).await;
    // test_threaded_watch().await;
}

pub async fn db_remove_tasks(db: &SocialsDb) {
    SocialsDb::delete_many(
        &BotTaskQuery::default(), &db.bots_tasks()
    ).await.expect("Some error while deleting");
}

pub async fn test_task_watch (db: &SocialsDb) {
    let action = TaskActionEnum::WatchAction(WatchAction {
        data: WatchTargetData {
            watch_count: 2,
            watch_seconds: 2,
            time_spread: 1, // 3600 - 60 minutes for task
            resource_link: "https://www.youtube.com/watch?v=jbsZRyNHliY".to_string(),
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

pub async fn create_test_watch_actions (count: u64) {
    let db = SocialsDb::new_test_instance()
        .await.unwrap();
    let action = TaskActionEnum::WatchAction(WatchAction {
        data: WatchTargetData {
            watch_count: 20,
            watch_seconds: 1,
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
    for _i in 0..count {
        let task = BotTask::create_from(&db, new_task.clone()).await;
        let t =  task.clone();
        let result = SocialsDb::insert_one(t, db.bots_tasks()).await;
        assert_eq!(result.is_ok(), true)
    }
}

pub async fn test_task_watch_db () {
    let db = SocialsDb::new_test_instance()
        .await.unwrap();

    let action = TaskActionEnum::WatchAction(WatchAction {
        data: WatchTargetData {
            watch_count: 20,
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
    task.make(&db).await;
}

pub async fn make_db_task () {
    let db = SocialsDb::new_test_instance()
        .await.unwrap();
    let mut query = BotTaskQuery::default();
    query.is_active()
        .is_browser()
        .top_old_updated();
    debug!("query is {:#?}", query);
    // debug!("query collect is {:#?}", query.collect_one_options());
    loop {
        let mut task = SocialsDb::find_one(&query, &db.bots_tasks())
            .await
            .expect("Error while trying to find task in db")
            .expect("Cant find task in db to make");

        task.make(&db).await;
        tokio::time::sleep(std::time::Duration::from_secs(3)).await
    }
}

pub async fn test_threaded_watch () {
    create_test_watch_actions(2).await;
    let worker1 = make_db_task();
    let worker2 = make_db_task();
    let workers = [worker1, worker2];
    futures::future::join_all(workers).await;
}
