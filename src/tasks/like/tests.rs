pub mod vk {
    use log::info;

    use crate::{db::SocialsDb, tasks::{TaskActionEnum, like::{LikeAction, LikeTargetData}, TaskActionType, BotTask, BotTaskCreate, TaskTarget}, social::SocialPlatform};

    #[tokio::test]
    pub async fn like_add() {
        env_logger::try_init().ok();
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
}


