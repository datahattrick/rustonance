
use std::error::Error;
use rustonance::client::Client;
use tracing::error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let subscriber = tracing_subscriber::fmt()
        // Use a more compact, abbreviated log format
        .compact()
        // Display source code file paths
        .with_file(true)
        // Display source code line numbers
        .with_line_number(true)
        // Display the thread ID an event was recorded on
        .with_thread_ids(true)
        // Don't display theve event's target (module path)
        .with_target(false)
        .finish(); 

    let _ = tracing::subscriber::set_global_default(subscriber);

    let mut rustonance = Client::default().await
        .map_err(|err| error!("Failed to implement Client: {}", err)).unwrap();

    tokio::spawn(async move {
        let _ = rustonance.start().await.map_err(|why| println!("Client ended: {:?}", why));
    });

    let _signal_err = tokio::signal::ctrl_c().await;
    println!("Received Ctrl-C, shutting down.");

    Ok(())
}
