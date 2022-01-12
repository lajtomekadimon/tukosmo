use postgres::{Client, NoTls, Error, row};

use crate::config::global::DB_AUTH_STRING;
use crate::i18n::t_error::t_error;


pub fn query_db_noparam(
    query_string: &str,
) -> Result<row::Row, Error> {

    match Client::connect(DB_AUTH_STRING, NoTls) {

        Ok(mut client) => match client.query_one(query_string, &[]) {
            Ok(row_result) => Ok(row_result),
            Err(e) => {
                // Debugging
                // TODO: Only in development
                println!("{}", &e);
                println!("{}", t_error(&e, "en").message);

                Err(e)
            },
        },

        Err(e) => {
            // Debugging
            // TODO: Only in development
            println!("CONNECTION ERROR: {}", &e);
            println!("CONNECTION ERROR: {}", t_error(&e, "en").message);

            Err(e)
        },

    }

}

