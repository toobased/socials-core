use crate::db::SocialsDb;

use crate::social::source::{SocialSourceQuery, SocialSource};


#[tokio::test]
pub async fn test_crud () {
    remove_social_sources().await;
    create_social_source().await;
    create_social_source_json().await;
    get_social_sources().await;
    find_social_source().await;
}

pub async fn remove_social_sources() {
    let db = SocialsDb::new_test_instance().await.unwrap();
    SocialsDb::delete_many(
        &SocialSourceQuery::default(), &db.social_sources()
    ).await.expect("Some error while deleting");
}

pub async fn create_social_source () {
    let db = SocialsDb::new_test_instance().await.unwrap();
    let source = SocialSource { 
        name: "random_name".to_string(),
        ..Default::default()
    };
    SocialsDb::insert_one(source, db.social_sources())
        .await.unwrap()
}

pub async fn create_social_source_json () {
    let db = SocialsDb::new_test_instance().await.unwrap();
    let source_raw = r#"{
        "name": "test_name",
        "platforms": [
            {"platform": "Vk", "source_link": "some_link_is_here"},
            {"platform": "Ok", "source_link": "another_link_is_here"}
        ]
    }"#;
    let source = serde_json::from_str::<SocialSource>(source_raw).expect("Cant parse social source from raw json");
    SocialsDb::insert_one(source, db.social_sources())
        .await.unwrap()
}

pub async fn get_social_sources () {
    let db = SocialsDb::new_test_instance().await.unwrap();
    SocialsDb::
        find(&SocialSourceQuery::default(), &db.social_sources())
        .await.unwrap();
}

pub async fn find_social_source () {
    let db = SocialsDb::new_test_instance().await.unwrap();
    let query = SocialSourceQuery {
        title: Some("test_name".to_string()),
        ..Default::default()
    };
    SocialsDb::find_one(&query, &db.social_sources())
        .await.unwrap().expect("Dont find social source?");
}

/*
async fn error_find_social_source () {
    let db = SocialsDb::new_test_instance().await.unwrap();
    let query = SocialSourceQuery {
        title: Some("non_existed_name".to_string()),
        ..Default::default()
    };
    SocialsDb::find_one(query, db.social_sources())
        .await.unwrap().expect("Dont find social source?");
}
*/
