[![Docs](https://img.shields.io/badge/docs-online-informational)](https://docs.rs/serverseeker)
![Crates.io](https://img.shields.io/crates/d/serverseeker)

# serverseeker-rs
A Rust wrapper for DAMcrafts ServerSeeker API

This project is currently work-in-progress!
Also, I'm not the best Rust coder, so the code is probably bad :/

Example:
```rust
// [dependencies]
// serverseeker = "^0.2"
// tokio = { version = "1.28.2", features = ["rt-multi-thread", "macros"] }

use serverseeker::ServerSeekerClient;

#[tokio::main]
async fn main() {
    // Initialize a ServerSeekerClient with your API key
    // How to get your API key: https://github.com/DAMcraft/ServerSeekerAPI-docs
    let ss = ServerSeekerClient::new("YOUR_API_KEY".to_string()).unwrap();
    // Get a list of cracked servers
    let servers = ss.servers(|f| f.cracked(true)).await.unwrap();
    // Print the IP of every server
    for server in servers {
        println!("IP: {}, cracked: {:?}", server.server, server.cracked);
    }
}

```
For more examples, take a look at the [wiki](https://github.com/Paddyk45/serverseeker-rs/wiki)
