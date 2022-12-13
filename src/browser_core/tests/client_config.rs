use log::info;
use crate::browser_core::BrowserCore;

#[tokio::test]
pub async fn test_client_config()  {
    env_logger::try_init().ok();
    log::set_max_level(log::LevelFilter::Debug);
    test_window_size().await;
}

pub async fn test_window_size () {
    info!("[test_window_size]");
    let browser = BrowserCore::init().await;
    let (w, h) = browser.client.get_window_size().await.unwrap();
    let (cw, ch) = browser.config.window_size();
    browser.close().await;
    info!("config: ({},{}), actual: ({},{})", cw, ch, w, h);
    assert_eq!((w, h), (cw, ch), "Actual window width & height not match config required");
}
