use std::env;

use serverseeker::*;

#[tokio::main]
async fn main() {
    let ss = ServerSeekerClient::new(env::var("SS_API_KEY").expect("Failed to get SS_API_KEY"));

    let stats = ss.stats().await.expect("Failed to get stats");
    let user_info = ss.user_info().await.expect("Failed to get user_info");
    dbg!(stats, user_info);

}
