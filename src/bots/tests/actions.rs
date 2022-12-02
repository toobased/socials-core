pub mod vk {
    use log::info;
    use vk_client::client::VkClient;

    use crate::{bots::Bot, social::SocialPlatform};

    #[tokio::test]
    async fn main_tests () {
        env_logger::try_init().ok();
        log::set_max_level(log::LevelFilter::Info);

        fetch_by_token().await;
    }

    async fn fetch_by_token () {
        let admin_client = VkClient::init_admin();
        let test_client = VkClient::init_test();
        let p = SocialPlatform::Vk;
        info!("[VK Fetch token] fetch admin by token (env var)");
        fetch_user_by_token(&p, &admin_client.access_token).await;
        info!("[VK Fetch token] fetch user by token (env var)");
        fetch_user_by_token(&p, &test_client.access_token).await;
    }

    async fn fetch_user_by_token(p: &SocialPlatform, token: &str) {
        let res = Bot::fetch_by_access_token(p.clone(), "").await;
        // info!("{} err result is {:#?}", m, res);
        assert_eq!(res.is_err(), true);
        let res = Bot::fetch_by_access_token(p.clone(), token).await;
        // info!("{} success result is {:#?}", m, res);
        assert_eq!(res.is_ok(), true);
    }
}
