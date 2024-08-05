
use std::error::Error;
use rustonance::client::Client;
pub mod error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let mut rustonance = Client::default().await?;

    tokio::spawn(async move {
        let _ = rustonance.start().await.map_err(|why| println!("Client ended: {:?}", why));
    });

    let _signal_err = tokio::signal::ctrl_c().await;
    println!("Received Ctrl-C, shutting down.");

    Ok(())
}
