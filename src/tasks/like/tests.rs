pub mod calc;
pub mod sleep;

pub mod vk {
    use log::info;

    use crate::{db::SocialsDb, tasks::{BotTaskQuery, like::LikeAction}, social::SocialPlatform, tests::db_helpers::{clean_tasks_db, clean_bots_db, clean_events_db, insert_test_bot, insert_like_task}};

    static ITEM_ID: &str = "";
    static OWNER_ID: &str = "";
    static RESOURCE_LINK_SUCESS: &str = "https://vk.com/kf_films?w=wall-211982694_1790";
    static RESOURCE_LINK_FAIL: &str = "asdfasdf";
    // https://vk.com/kf_films?w=wall-211982694_1790
    static PLATFORM: SocialPlatform = SocialPlatform::Vk;

    #[tokio::test]
    pub async fn test_like () {
        info!("-- VK TEST LIKE --");
        // db
        let db = SocialsDb::new_test_instance().await.unwrap();
        // logging
        env_logger::try_init().ok();
        log::set_max_level(log::LevelFilter::Info);

        make_like_task(&db, RESOURCE_LINK_FAIL, 5, true, false).await;
        make_like_task(&db, RESOURCE_LINK_SUCESS, 1, false, true).await;
        make_like_task(&db, RESOURCE_LINK_SUCESS, 5, false, false).await;
    }

    pub async fn make_like_task(
        db: &SocialsDb, resource_link: &str, count: u64,
        should_fail: bool, should_finish: bool
    ) {
        let token: String = std::env::var("vk_test_access_token").unwrap();
        // clean dbs
        clean_tasks_db(&db).await;
        clean_bots_db(&db).await;
        clean_events_db(&db).await;

        // inserting data
        for _i in 0..10 { insert_test_bot(&db, &token, &PLATFORM).await; }
        // insert tasks
        for _i in 0..1 { insert_like_task(&db, ITEM_ID, OWNER_ID, resource_link, &PLATFORM, count, 0).await; }
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
        let q = BotTaskQuery::new();
        let task = SocialsDb::find_one(&q, &db.bots_tasks()).await.unwrap().unwrap();
        if should_fail { assert_eq!(task.is_error(), true) }
        if should_finish { assert_eq!(task.is_done(), true)}
        if !should_fail {
            // ensure that specified `count` = actual taks metrics
            let action: LikeAction = task.action.try_into().unwrap();
            assert_eq!(action.stats.like_count == count, true);
        };
    }
}
