use std::env;

use serverseeker::*;

#[tokio::main]
async fn main() {
    let ss = ServerSeekerClient::new(env::var("SS_API_KEY").expect("Failed to get SS_API_KEY"));

    // Initialize the builder
    let mut builder = ServersBuilder::default();

    // Only cracked servers
    builder.cracked(true);

    // Only german servers
    builder.country_code("DE");

    // Only servers that have "SMP" in their MOTD
    builder.description("SMP");

    // Fetch results
    let servers = ss.servers(&builder).await.unwrap();

    // Print the IP of every server
    dbg!(servers);
}
