use tokio_postgres::{connect, NoTls, Error, row};
use tokio;
use postgres_types::ToSql;

use crate::config::global::Config;
use crate::i18n::t_error::t_error;


pub trait QueryFunction {
    fn query(&self) -> &str;
}


pub async fn query_db<RequestType: QueryFunction + ToSql + std::marker::Sync>(
    config: &Config,
    r: RequestType,
) -> Result<row::Row, Error> {

    let server_mode = config.server.mode.as_str();

    match connect(&config.dbauth, NoTls).await {

        Ok((client, connection)) => {
            // The connection object performs the actual communication with
            // the database, so spawn it off to run on its own
            tokio::spawn(async move {
                if let Err(e) = connection.await {
                    eprintln!("Connection error: {}", e);
                }
            });

            match client.query_one(
                r.query(),
                &[&r,]
            ).await {
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

