pub mod vk {
    use log::info;

    use crate::{db::SocialsDb, tasks::{TaskActionEnum, like::{LikeAction, LikeTargetData}, TaskActionType, BotTask, BotTaskCreate, TaskTarget, BotTaskQuery}, social::SocialPlatform};

    // #[tokio::test]
    pub async fn like_add() {
        env_logger::try_init().ok();
        log::set_max_level(log::LevelFilter::Info);
        let db = SocialsDb::new_test_instance().await.unwrap();
        let action = TaskActionEnum::LikeAction(LikeAction {
            target: TaskTarget::Post,
            data: LikeTargetData {
                like_count: 1,
                owner_id: Some("-211982694".to_string()),
                item_id: Some("1250".to_string()),
                time_spread: 0,
                // resource_link: "https://www.youtube.com/watch?v=zuL55W3Ivtk&t=3s".to_string(),
                ..Default::default()
            },
            ..Default::default()
        });
        let new_task = BotTaskCreate {
            action_type: TaskActionType::Like,
            platform: SocialPlatform::Vk,
            action,
            ..Default::default()
        };
        let mut task = BotTask::create_from(&db, new_task).await;
        info!("task is {:#?}", task);
        task.make_v2(&db).await;
        // SocialsDb::insert_one(task.clone(), db.bots_tasks()).await.expect("Cant insert task into db");
    }

    pub async fn clean_tasks_db (db: &SocialsDb) {
        let q = BotTaskQuery::new();
        SocialsDb::delete_many(&q, &db.bots_tasks()).await.unwrap();
    }

    #[tokio::test]
    pub async fn make_like_task() {
        env_logger::try_init().ok();
        log::set_max_level(log::LevelFilter::Info);
        let db = SocialsDb::new_test_instance().await.unwrap();
        clean_tasks_db(&db).await;
        insert_like_task(&db).await;

        for _i in 0..2 {
            let mut q = BotTaskQuery::new();
            q
                .not_browser()
                .is_active()
                .top_old_updated();
            let mut task = SocialsDb::find_one(&q, &db.bots_tasks())
                .await.unwrap().unwrap();
            task.make(&db).await;
        }

    }

    pub async fn insert_like_task(db: &SocialsDb) {
        let action = crate::tasks::like::LikeAction {
            data: crate::tasks::like::LikeTargetData {
                like_count: 1,
                item_id: Some("1250".to_string()),
                owner_id: Some("-211982694".to_string()),
                ..Default::default()
            },
            ..Default::default()
        };

        let new_task = BotTaskCreate {
            is_active: true,
            title: "testing".to_string(),
            platform: crate::social::SocialPlatform::Vk,
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
}


