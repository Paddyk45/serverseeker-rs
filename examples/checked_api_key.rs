use std::env;

use serverseeker::*;

#[tokio::main]
async fn main() {
    let ss =
        ServerSeekerClient::new_checked(env::var("SS_API_KEY").expect("Failed to get SS_API_KEY"))
            .await;
    println!(
        "Your api_key is {}",
        match ss {
            Ok(_) => {
                "valid"
            }
            Err(_) => {
                "invalid"
            }
        }
    )
}
