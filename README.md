# Archived because ServerSeeker is now owned by a griefing group

[![Docs](https://img.shields.io/badge/docs-online-informational)](https://docs.rs/serverseeker)![Crates.io](https://img.shields.io/crates/d/serverseeker)

# serverseeker-rs
A Rust wrapper for DAMcrafts ServerSeeker API

# What is ServerSeeker?
ServerSeeker is a bot that scans the entire internet for minecraft servers and collects data from the [Server List Ping](https://wiki.vg/Server_List_Ping) (SLP). It also checks if the server is cracked

#
This project is currently work-in-progress!
Also, I'm not the best Rust coder, so the code is probably bad :/

# Example
```rust
// [dependencies]
// serverseeker = "^0.3"
// tokio = { version = "^1.28.2", features = ["rt-multi-thread", "macros"] }

use serverseeker::*;

#[tokio::main]
async fn main() {
    // Initialize a ServerSeekerClient with your API key
    // How to get your API key: https://github.com/DAMcraft/ServerSeekerAPI-docs
    let ss = ServerSeekerClient::new("YOUR_API_KEY");

    // Initialize the builder
    let mut builder = ServersBuilder::default();
    builder.cracked(true);
    builder.country_code("DE");

    // Fetch results
    let servers = ss.servers(&b).await.unwrap();

    // Print the IP of every server
    for server in servers {
        println!("IP: {}, cracked: {:?}", server.server, server.cracked);
    }
}

```
For more examples, take a look at /examples
