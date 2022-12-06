use std::time::{SystemTime, Duration};

use log::info;

use crate::tasks::{like::LikeAction, BotTaskCreate, TaskActionType, TaskActionEnum, BotTask, TaskAction};

#[test]
pub fn test_calc_next_time_run () {
    env_logger::try_init().ok();
    log::set_max_level(log::LevelFilter::Info);
    info!("-- `test_calc_next_time_run` --");
    // 10 need, 0 liked, 1 second left. Should be 0 <= x <= 3
    calc_need_next_time_run_tester(10, 0, 1, false, 0, 3);
    // 10 need, 9 liked, 2 minutes. Should be 100 <= x <= 120
    calc_need_next_time_run_tester(10, 9, 2 * 60, false, 100, 120);
    // 10 need, 10 liked, 2 minutes. Should be None
    calc_need_next_time_run_tester(10, 10, 2 * 60, true, 0, 0);
    // 10 need, 12 liked, 2 minutes. Should be None
    calc_need_next_time_run_tester(10, 12, 2 * 60, true, 0, 0);
}

// #[test]
pub fn test_calc_need_do () {
    env_logger::try_init().ok();
    log::set_max_level(log::LevelFilter::Info);
    info!("-- `test_calc_next_time_run` --");
    // 10 need, 0 liked, 1 second left. Should be 3 <= x <= 5
    calc_need_do_tester(10, 0, 1, 3, 5);
    // 10 need, 0 liked, 2 minutes left. Should be 1 <= x <= 3
    calc_need_do_tester(10, 0, 2 * 60, 1, 3);
    // 30 need, 0 liked, 2 minutes left. Should be 1 <= x <= 3
    calc_need_do_tester(30, 0, 2 * 60, 1, 3);
    // 0 need, 5 liked, 2 minutes left. Should be 0 <= x <= 0
    calc_need_do_tester(0, 5, 2 * 60, 0, 0);
}

pub fn calc_need_do_tester (
    need_do: u64,
    done: u64,
    time_spread: u64,
    test_min: u64,
    test_max: u64
) {
    let mut action = LikeAction { ..Default::default() };
    action.data.like_count = need_do;
    action.stats.like_count = done;
    action.data.time_spread = time_spread;
    let new_task = BotTaskCreate {
        action_type: TaskActionType::Like,
        action: TaskActionEnum::LikeAction(action.clone()),
        ..Default::default()
    };
    let task = BotTask::from(new_task);
    let predict_need_do = action.calc_need_do_now(&task);
    assert_eq!(predict_need_do <= need_do, true, "`predict_need_do` should be <= `need_do - done`");
    info!(
        "need: {} done: {} time_spread: {}, prediction: {}, should be: {} <= X <= {}",
        need_do, done, time_spread, predict_need_do, test_min, test_max
    );
    assert_eq!(
        predict_need_do >= test_min && predict_need_do <= test_max, true,
        "prediction not match with awaitable result"
    );
}

pub fn calc_need_next_time_run_tester (
    need_do: u64,
    done: u64,
    time_spread: u64,
    is_none: bool,
    test_min: u64,
    test_max: u64
) {
    let mut action = LikeAction { ..Default::default() };
    action.data.like_count = need_do;
    action.stats.like_count = done;
    action.data.time_spread = time_spread;
    let new_task = BotTaskCreate {
        action_type: TaskActionType::Like,
        action: TaskActionEnum::LikeAction(action.clone()),
        ..Default::default()
    };
    let mut task = BotTask::from(new_task);
    action.calc_next_time_run(&mut task);
    let prediction = task.next_run_time;
    match is_none {
        true => assert_eq!(
            prediction.is_none(),
            true,
            "prediction was awaited to be `None`"
        ),
        false => {
            let prediction = prediction.unwrap().duration_since(SystemTime::now()).unwrap_or(Duration::from_secs(1)).as_secs();
            info!(
                "need: {} done: {} time_spread: {}, prediction: {}, should be: {} <= X <= {}",
                need_do, done, time_spread, prediction, test_min, test_max
            );
            assert_eq!(
                prediction >= test_min && prediction <= test_max, true,
                "prediction not match with awaitable result"
            );
        }
    }

}
