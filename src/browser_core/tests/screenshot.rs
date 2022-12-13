use log::info;
use crate::browser_core::BrowserCore;

#[tokio::test]
pub async fn test_browser_screenshot ()  {
    env_logger::try_init().ok();
    log::set_max_level(log::LevelFilter::Debug);
    test_screenshot_base().await;
}

pub async fn test_screenshot_base () {
    info!("[test_screenshot_base]");
    let browser = BrowserCore::init().await;
    // let client = &browser.client;
    browser.save_shot("test_shot.png").await.unwrap();
    browser.close().await;
}
