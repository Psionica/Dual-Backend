extern crate anyhow;

mod nlp;
mod server;
mod utils;

use crate::server::*;

#[tokio::main]
async fn main() {
    serve().await;
}
