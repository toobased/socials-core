use std::time::Duration;

use log::{info, debug};

use crate::browser_core::BrowserCore;

#[tokio::test]
pub async fn test_browser_processes ()  {
    env_logger::init();
    info!("Run test browser processes");
    // test_zombie().await;
    test_driver_init_close().await;
}

pub async fn test_driver_init_close () {
    let core = BrowserCore::init().await;
    debug!("Driver initialized. Sleep for 5 secs");
    tokio::time::sleep(Duration::from_secs(5)).await;
    core.close().await;
    debug!("Driver closed");
}

pub async fn test_zombie () {
    for n in 1..4 {
        let actions = (0..2).map(|_| dumb_session(n));
        futures::future::join_all(actions).await;
    }
    info!("End of futures cycle");
    let after_sleep: u64 = 30;
    info!("Ended browser sessions, sleep for {}", after_sleep);
    tokio::time::sleep(Duration::from_secs(after_sleep)).await;
}

pub async fn dumb_session (session_num: i32) {
    let core = BrowserCore::init().await;
    let client = &core.client;
    go_dumb_page(client).await;
    let secs_parsed = u64::try_from(session_num).unwrap();
    let secs = secs_parsed + (secs_parsed);
    let sleep = Duration::from_secs(secs);
    info!("Fall alseep for {}", secs);
    tokio::time::sleep(sleep).await;
    core.close().await;
}

pub async fn go_dumb_page (client: &fantoccini::Client) {
    // let dumb_page = "https://google.com";
    let dumb_page = "https://zen.yandex.ru/video/watch/6313115829a9f66df8aeced0";
    client.goto(dumb_page)
        .await.expect("Cant go to dump page");
}
