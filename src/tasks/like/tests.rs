pub mod vk {
    use log::info;

    use crate::{db::{SocialsDb, DbActions}, tasks::{TaskActionEnum, TaskActionType, BotTask, BotTaskCreate, TaskTarget, BotTaskQuery}, social::SocialPlatform, bots::{query::BotQuery, BotCreate, Bot}};

    static ITEM_ID: &str = "1250";
    static OWNER_ID: &str = "-211982694";
    static RESOURCE_LINK: &str = "asdfasdf";


    #[tokio::test]
    pub async fn make_like_task() {
        // logging
        env_logger::try_init().ok();
        log::set_max_level(log::LevelFilter::Info);

        // db
        let db = SocialsDb::new_test_instance().await.unwrap();

        // clean dbs
        clean_tasks_db(&db).await;
        clean_bots_db(&db).await;
        clean_events_db(&db).await;
        info!("db cleaned");

        // inserting data
        insert_test_bot(&db).await;

        for _i in 0..2 {
            insert_like_task(&db).await;
        }

        // making task
        for _i in 0..6 {
            let mut q = BotTaskQuery::new();
            q
                .not_sleep()
                .not_browser()
                .is_active()
                .top_old_updated();
            let task = SocialsDb::find_one(&q, &db.bots_tasks())
                .await.unwrap();
            match task {
                None => info!("No task to do"),
                Some(mut t) => t.make(&db).await
            };
        }

    }

    pub async fn clean_tasks_db (db: &SocialsDb) {
        let q = BotTaskQuery::new();
        SocialsDb::delete_many(&q, &db.bots_tasks()).await.unwrap();
    }

    pub async fn clean_bots_db (db: &SocialsDb) {
        let q = BotQuery::new();
        SocialsDb::delete_many(&q, &db.bots()).await.unwrap();
    }

    pub async fn clean_events_db (db: &SocialsDb) {
        let q = BotQuery::new();
        SocialsDb::delete_many(&q, &db.action_events()).await.unwrap();
    }

    pub async fn insert_test_bot(db: &SocialsDb) {
        let token = Some(std::env::var("vk_test_access_token").unwrap());
        let new_bot = BotCreate {
            access_token: token,
            platform: SocialPlatform::Vk,
            make_ready: true,
            ..Default::default()
        };
        let mut bot = Bot::create_from(&db, new_bot).await.unwrap();
        bot.insert_db(&db).await.unwrap();
    }

    pub async fn insert_like_task(db: &SocialsDb) {
        let action = crate::tasks::like::LikeAction {
            target: TaskTarget::Post,
            data: crate::tasks::like::LikeTargetData {
                like_count: 1,
                item_id: Some(ITEM_ID.to_string()),
                owner_id: Some(OWNER_ID.to_string()),
                resource_link: Some(RESOURCE_LINK.to_string()),
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


