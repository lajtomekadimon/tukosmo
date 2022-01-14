use postgres::{Client, NoTls, Error, row};

use crate::config::global::Config;
use crate::i18n::t_error::t_error;


pub fn query_db_noparam(
    config: &Config,
    query_string: &str,
) -> Result<row::Row, Error> {

    let server_mode = config.server.mode.as_str();

    match Client::connect(&config.dbauth, NoTls) {

        Ok(mut client) => match client.query_one(query_string, &[]) {
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

