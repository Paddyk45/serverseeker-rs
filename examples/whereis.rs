use std::env;

use serverseeker::*;

#[tokio::main]
async fn main() {
    let ss = ServerSeekerClient::new(env::var("SS_API_KEY").expect("Failed to get SS_API_KEY"));

    // Initialize the builder
    let mut builder = WhereisBuilder::default();

    // Search player "DAMcraft"
    builder.name("DAMcraft");

    // Fetch results
    let servers = ss.whereis(&mut builder).await.unwrap();

    // Print the IP of every server
    dbg!(servers);
}
