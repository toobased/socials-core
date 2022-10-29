use async_trait::async_trait;
use log::info;
use vk_client::client::VkClient;
use vk_client::{likes, media, client::response::VkError};

use crate::bots::errors::BotError;
use crate::bots::query::BotQuery;
use crate::db::DbActions;
use crate::social::SocialPlatform;
use crate::tasks::{TaskActionEnum, TaskActionType};
use crate::tasks::errors::TaskError;
use crate::tasks::events::ActionEvent;
use crate::{tasks::{like::LikeAction, BotTask, TaskAction}, db::SocialsDb};

use super::{SocialCore, SocialCoreConfig};

pub struct VkCoreConfig { }

impl Default for VkCoreConfig {
    fn default() -> Self {
        Self {}
    }
}

impl SocialCoreConfig for VkCoreConfig {}

pub struct VkCore {
    pub config: VkCoreConfig
}

pub enum VkCoreParsedError {
    Task(TaskError),
    Bot(BotError)
}

impl VkCore {
    pub fn new () -> Self { Self::default() }
    pub fn make_client(token: &str) -> VkClient { VkClient::init(token) }

    pub fn parse_error(e: &VkError) -> VkCoreParsedError {
        let m = Some(e.merge_msg()); let d = e.log.clone(); let c = e.error_code;
        // parse user errors
        let usr_err = match c {
            5 => BotError::auth(m, d),
            7|15|17|24|30|200|201|203|500|600 => BotError::access_denied(m, d),
            18 => BotError::ban(m, d),
            14 => BotError::captcha(m, d),
            _ => BotError::dummy()
        };
        if !usr_err.is_dummy() { return VkCoreParsedError::Bot(usr_err)}
        let m = Some(e.merge_msg()); let d = e.log.clone();
        // parse task errors
        let task_err = match c {
            1 => TaskError::unknown(m, d),
            2|3|4|6|8|9|10|16|20|21|23|28|29|100|101|150|603 =>
                TaskError::action_error(m, d),
            _ => TaskError::unknown(m, d)
        };
        VkCoreParsedError::Task(task_err)
    }
}

impl Default for VkCore {
    fn default() -> Self {
        Self {
            config: VkCoreConfig::default()
        }
    }
}

#[async_trait]
impl SocialCore for VkCore {
    type CoreConfig = VkCoreConfig;

    fn config(&self) -> &VkCoreConfig { &self.config }

    fn info(&self) -> String { "VkCore".to_string() }

    async fn like(&self, action: LikeAction, task: &mut BotTask, db: &SocialsDb) {
        let need_do = action.calc_need_do_now(task);
        let owner_id = action.data.owner_id.unwrap_or("".to_string()); // TODO
        let item_id = action.data.item_id.unwrap_or("".to_string()); // TODO

        // getting bots for task
        let mut bots_query = BotQuery::new();
        bots_query
            .is_ready()
            .is_awake_for(TaskActionType::Like)
            .has_token()
            .top_old_used()
            .with_platform(SocialPlatform::Vk)
            .exclude_ids(action.stats.bots_used)
            .limit(i64::try_from(need_do).unwrap());
        let mut bots = SocialsDb::find(&bots_query, &db.bots())
            .await.unwrap();

        if bots.items.len() == 0 {
            info!("No bots for task found");
            task
                .sleep_no_bots(None)
                .update_db(&db).await.unwrap();
            return
        }

        for bot in bots.items.iter_mut() {
            let bot_token = bot.access_token.as_ref().unwrap(); // TODO
            let client = VkCore::make_client(&bot_token);
            let query = likes::query::IsLikedQuery {
                media_type: media::POST.to_string(),
                owner_id: owner_id.clone(),
                item_id: item_id.clone(),
                ..Default::default()
            };
            let result = likes::is_liked(&client, query).await;
            match result {
                Err(vk_err) => {
                    match VkCore::parse_error(&vk_err) {
                        VkCoreParsedError::Task(e) => {
                            task
                                .get_fresh(&db).await.unwrap() // TODO
                                .process_error(e)
                                .update_db(&db).await.unwrap(); // TODO
                        },
                        VkCoreParsedError::Bot(e) => {
                            bot.get_fresh(&db).await.unwrap() // TODO
                                .process_error(e)
                                .update_db(&db).await.unwrap(); // TODO
                        }
                    }
                    break;
                },
                Ok(r) => {
                    if r.liked > 0 {
                        info!("[Already liked] task: {}, bot: {}", task.id, bot.id);
                        task.get_fresh(&db).await.unwrap();
                        let mut action: LikeAction = task.action.clone()
                            .try_into().ok().unwrap();
                        action.add_used_bot(&bot.id);
                        task.action = TaskActionEnum::LikeAction(action);
                        task.update_db(&db).await.unwrap();
                        continue
                    }
                    let query = likes::query::AddLikeQuery {
                        media_type: media::POST.to_string(),
                        owner_id: owner_id.clone(),
                        item_id: item_id.clone(),
                        ..Default::default()
                    };
                    let result = likes::add(&client, query).await;

                    match result {
                        Err(vk_err) => {
                            match VkCore::parse_error(&vk_err) {
                                VkCoreParsedError::Task(e) => {
                                    task
                                        .get_fresh(&db).await.unwrap() // TODO
                                        .process_error(e)
                                        .update_db(&db).await.unwrap(); // TODO
                                },
                                VkCoreParsedError::Bot(e) => {
                                    bot.get_fresh(&db).await.unwrap() // TODO
                                        .process_error(e)
                                        .update_db(&db).await.unwrap(); // TODO
                                }
                            }
                            break;
                        },
                        Ok(_r) => {
                            info!("bot {} added like to {} task", bot.id, task.id);
                            task.get_fresh(db).await.unwrap(); // TODO
                            let mut action: LikeAction = task.action.clone()
                                .try_into().ok().unwrap(); // TODO handle error
                            action.stats.like_count += 1;
                            action.add_used_bot(&bot.id);
                            task.action = TaskActionEnum::LikeAction(action.clone());
                            task.update_db(db).await.unwrap(); // TODO
                            // adding event action
                            let mut event = ActionEvent::from_task(&task);
                            event
                                .set_amount(1)
                                .set_bot_id(bot.id.clone())
                                .set_platform(task.platform.clone())
                                .set_action_type(action.action_type())
                                .insert_db(&db).await.unwrap();
                            // TODO bot delay
                            bot
                                .get_fresh(&db).await.unwrap() // TODO safe
                                .after_action_sleep(&action, &db).await
                                .update_db(&db).await.unwrap(); // TODO handle safe
                        }
                    }

                }
            }
        }
    }
}
