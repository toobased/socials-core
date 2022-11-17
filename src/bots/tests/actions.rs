pub mod vk {
    use log::info;
    use vk_client::client::VkClient;

    use crate::{bots::Bot, social::SocialPlatform};

    #[tokio::test]
    async fn main_tests () {
        env_logger::try_init().ok();
        log::set_max_level(log::LevelFilter::Info);
        fetch_admin_user_by_token().await;
    }
    async fn fetch_admin_user_by_token() {
        let m: &str = "[fetch_admin_user_by_token]";
        let client = VkClient::init_admin();
        let res = Bot::fetch_by_access_token(
            SocialPlatform::Vk,
            ""
        ).await;
        info!("{} err result is {:#?}", m, res);
        assert_eq!(res.is_err(), true);
        let res = Bot::fetch_by_access_token(
            SocialPlatform::Vk,
            &client.access_token
        ).await;
        info!("{} success result is {:#?}", m, res);
        assert_eq!(res.is_ok(), true);
    }
}
