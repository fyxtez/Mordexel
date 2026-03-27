use chrono::{Datelike, Utc};
use tokio::time::{Duration, interval};
use tracing::info;

use crate::execution_policy::ExecutionPolicy;

pub async fn run(policy: ExecutionPolicy) {
    info!("policy_logger started");

    // Log immediately on boot
    policy.log_todays_plan();

    let mut last_logged_day = Utc::now().ordinal();
    let mut ticker = interval(Duration::from_secs(60));

    loop {
        ticker.tick().await;

        let today = Utc::now().ordinal();

        if today != last_logged_day {
            info!("new day detected, logging execution plan");
            policy.log_todays_plan();
            last_logged_day = today;
        }
    }
}
