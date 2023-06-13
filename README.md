# serverseeker-rs
A wrapper for DAMcrafts ServerSeeker API, written in rust

This project is currently work-in-progress!
Also, I'm not the best Rust coder, so the code is bad :/

Example:
```rust
// [dependencies]
// serverseeker = { git = "https://github.com/Paddyk45/serverseeker-rs" }
// tokio = { version = "1.28.2", features = ["rt-multi-thread"] }
// chrono = "0.4.26"

use serverseeker::ServerSeekerClient;

#[tokio::main]
async fn main() {
    // How to get your API key: https://github.com/DAMcraft/ServerSeekerAPI-docs
    let ss = ServerSeekerClient::new("YOUR_API_KEY".to_string()).unwrap();
    let servers = ss.servers(|f| f.cracked(true)).await.unwrap();
    for server in servers {
        println!("Server found: {}, cracked: {}", server.server, server.cracked);
    }
}

```
