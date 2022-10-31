use log::info;

use crate::social::{SocialPlatform, post::SocialPost};

// #[test]
pub fn test_parse_url() {
    env_logger::try_init().ok();
    log::set_max_level(log::LevelFilter::Info);

    // 1 - valid case
    let valid_link = "https://vk.com/kf_films?w=wall-211982694_1353";
    test_parse_url_vk_valid(valid_link);

    // 2 - valid case
    let valid2 = "https://vk.com/kf_films?w=wall-211982694_1231&q=any";
    test_parse_url_vk_valid(valid2);

    // 1 - invalid case
    let invalid1 = "https://vk.com/kf_films?w=wall-211982694";
    test_parse_url_vk_invalid(invalid1);

    // 2 - invalid case
    let invalid2 = "https://vk.com/im?sel=557036603&z=video-211982694_456239447%2Fc24d5c4d0bc8f19c0a";
    test_parse_url_vk_invalid(invalid2);
}

#[tokio::test]
pub async fn test_get_post_by_url () {
    env_logger::try_init().ok();
    log::set_max_level(log::LevelFilter::Info);

    let url = "https://vk.com/kf_films?w=wall-211982694_1353";
    let res = SocialPost::get_post_by_url(&SocialPlatform::Vk, url).await;
    info!("res is {:#?}", res);
    assert!(res.is_ok());
}

// #[tokio::test]
pub async fn test_get_post_by_data () {
    env_logger::try_init().ok();
    log::set_max_level(log::LevelFilter::Info);

    let link1 = "https://vk.com/kf_films?w=wall-211982694_1353";
    vk_get_post_by_data(link1).await;
}

pub async fn vk_get_post_by_data(url: &str) {
    let data = SocialPost::parse_data_from_url(&SocialPlatform::Vk, url).unwrap();
    info!("parse data iss {:#?}", data);
    let res = SocialPost::get_post_by_data(&SocialPlatform::Vk, &data).await;
    info!("get post res is {:#?}", res);
}

pub fn test_parse_url_vk_valid(url: &str) {
    info!("Start parsing valid case");
    let parse_res = SocialPost::parse_data_from_url( &SocialPlatform::Vk, url);
    assert!(parse_res.is_ok());
    info!("link is: {} parsed result is {:#?}", url, parse_res);
}
pub fn test_parse_url_vk_invalid(url: &str) {
    info!("Start parsing invalid case");
    let parse_res = SocialPost::parse_data_from_url( &SocialPlatform::Vk, url);
    assert!(parse_res.is_err());
    info!("link is: {} parsed result is {:#?}", url, parse_res);
}
