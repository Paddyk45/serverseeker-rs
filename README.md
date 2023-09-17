[![Docs](https://img.shields.io/badge/docs-online-informational)](https://docs.rs/serverseeker)
![Crates.io](https://img.shields.io/crates/d/serverseeker)

# serverseeker-rs
A Rust wrapper for DAMcrafts ServerSeeker API

This project is currently work-in-progress!
Also, I'm not the best Rust coder, so the code is probably bad :/

Example:
```rust
// [dependencies]
// serverseeker = "^0.3"
// tokio = { version = "^1.28.2", features = ["rt-multi-thread", "macros"] }

use serverseeker::*;

#[tokio::main]
async fn main() {
    // Initialize a ServerSeekerClient with your API key
    // How to get your API key: https://github.com/DAMcraft/ServerSeekerAPI-docs
    let ss = ServerSeekerClient::new("YOUR_API_KEY".to_string());

    // Initialize the builder
    let mut builder = ServersBuilder::default();
    b.cracked(true);
    b.country_code("DE".to_string());

    // Fetch results
    let servers = ss.servers(&mut b).await.unwrap();

    // Print the IP of every server
    for server in servers {
        println!("IP: {}, cracked: {:?}", server.server, server.cracked);
    }
}

```
For more examples, take a look at /examples
