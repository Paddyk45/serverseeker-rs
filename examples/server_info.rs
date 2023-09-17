use std::env;

use serverseeker::*;

#[tokio::main]
async fn main() {
    let ss = ServerSeekerClient::new(env::var("SS_API_KEY").expect("Failed to get SS_API_KEY"));

    // Initialize the builder
    let mut builder = ServerInfoBuilder::default();

    // Set the ip to 45.135.201.120
    builder.ip("45.135.201.120".to_string());

    // Set the port to 25565 (not required if the server port is 25565)
    builder.port(25565);

    // Fetch results
    let info = ss.server_info(&mut builder).await.unwrap();

    // Print the server info
    dbg!(info);
}