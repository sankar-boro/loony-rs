//! Publish to a redis channel example.
//!
//! A simple client that connects to a loony-redis server, and
//! publishes a message on `foo` channel
//!
//! You can test this out by running:
//!
//!     cargo run --bin loony-redis-server
//!
//! Then in another terminal run:
//!
//!     cargo run --example sub
//!
//! And then in another terminal run:
//!
//!     cargo run --example pub

use loony_redis::{clients::Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Open a connection to the loony-redis address.
    let mut client = Client::connect("127.0.0.1:6379").await?;

    // publish message `bar` on channel foo
    client.publish("foo", "bar".into()).await?;

    Ok(())
}
