use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};
use rand::Rng;
use futures::executor;

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

use crate::server::new_server::new_server;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // AUTH COOKIE
    // -----------
    // Generate a random 32 byte key. Note that it's important to use a
    // unique private key for every project. Anyone with access to the
    // key can generate authentication cookies for any user!
    let private_key: [u8; 32] = rand::thread_rng().gen::<[u8; 32]>();

    let mut start_n: i64 = 0;
    let man_restart = Arc::new(Mutex::new(false));

    loop {
        start_n += 1;

        // Create a channel
        let (tx, rx) = mpsc::channel::<()>();

        // Start server as normal but don't .await after .run() yet
        let server = new_server(
            tx,
            &private_key,
            &start_n,
        );

        // Clone the Server handle
        let srv = server.clone();
        let manual_restart_child = Arc::clone(&man_restart);
        thread::spawn(move || {
            // Wait for shutdown signal
            rx.recv().unwrap();

            // Indicate that the server has to restart
            let mut is_manual_restart_child =
                manual_restart_child.lock().unwrap();
            *is_manual_restart_child = true;

            // Stop server gracefully
            executor::block_on(srv.stop(true))
        });

        // Run server
        match server.await {
            Ok(_) => {
                let manual_restart_end = Arc::clone(&man_restart);
                let mut has_to_restart = manual_restart_end.lock().unwrap();
                if *has_to_restart {
                    *has_to_restart = false;
                    continue;
                } else {
                    break;
                }
            },
            Err(e) => {
                panic!("ACTIX SERVER ERROR: {}", e);
            }
        };

    }

    Ok(())

}

