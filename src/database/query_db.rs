use postgres::{Client, NoTls, Error, row};
use postgres_types::ToSql;

use crate::config::global::Config;
use crate::i18n::t_error::t_error;


pub trait QueryFunction {
    fn query(&self) -> &str;
}


pub fn query_db<RequestType: QueryFunction + ToSql + std::marker::Sync>(
    config: &Config,
    r: RequestType,
) -> Result<row::Row, Error> {

    let server_mode = config.server.mode.as_str();

    match Client::connect(&config.dbauth, NoTls) {

        Ok(mut client) => match client.query_one(
            r.query(),
            &[&r,]
        ) {
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

