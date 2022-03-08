use tokio_postgres::{connect, NoTls, Error, row};
use tokio;

use crate::config::global::Config;
use crate::i18n::t_error::t_error;


pub async fn query_db_noparam(
    config: &Config,
    query_string: &str,
) -> Result<row::Row, Error> {

    let server_mode = config.server.mode.as_str();

    // Connect to the database
    match connect(&config.dbauth, NoTls).await {

        Ok((client, connection)) => {
            // The connection object performs the actual communication with
            // the database, so spawn it off to run on its own
            tokio::spawn(async move {
                if let Err(e) = connection.await {
                    eprintln!("Connection error: {}", e);
                }
            });

            match client.query_one(query_string, &[]).await {
                Ok(row_result) => Ok(row_result),
                Err(e) => match server_mode {
                    "development" => {
                        // Debugging
                        println!("{}", &e);
                        println!("{}", t_error(&e, "en").message);
                        Err(e)
                    },
                    _ => Err(e),
                },
            }
        },

        Err(e) => match server_mode {
            "development" => {
                // Debugging
                println!("{}", &e);
                println!("{}", t_error(&e, "en").message);
                Err(e)
            },
            _ => Err(e),
        },

    }

}

