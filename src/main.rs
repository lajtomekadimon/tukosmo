use tokio;
use rand::Rng;

mod config;
mod database;
mod files;
mod handlers;
mod i18n;
mod markdown;
mod minifiers;
mod routes;
mod server;
mod templates;

use crate::server::new_server::{Handle, new_server};


#[tokio::main]
async fn main() -> std::io::Result<()> {

    // AUTH COOKIE
    // -----------
    // Generate a random 32 byte key. Note that it's important to use a
    // unique private key for every project. Anyone with access to the
    // key can generate authentication cookies for any user!
    let private_key: [u8; 32] = rand::thread_rng().gen::<[u8; 32]>();

    let mut start_n: i64 = 0;

    loop {
        start_n += 1;

        let handle = Handle::new();

        // Start server as normal but don't .await after .run() yet
        let srv = new_server(
            handle.clone(),
            //tx.clone(),
            private_key.clone(),
            start_n.clone(),
        ).await;

        // Obtain handle to server
        handle.replace(srv.handle()).await;

        // Spawn server as Tokio task to start processing connections
        let hnd = tokio::spawn(srv);

        // Wait until server stops, and then...
        match hnd.await {
            Ok(_) => if let None = handle.0.lock().unwrap().take() {
                continue;
            } else {
                break;
            },
            Err(e) => {
                panic!("ACTIX SERVER ERROR: {}", e);
            }
        };

    }

    Ok(())

}

