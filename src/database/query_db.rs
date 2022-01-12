use postgres::{Client, NoTls, Error, row};
use postgres_types::ToSql;

use crate::config::global::DB_AUTH_STRING;
use crate::i18n::t_error::t_error;


pub trait QueryFunction {
    fn query(&self) -> &str;
}


pub fn query_db<RequestType: QueryFunction + ToSql + std::marker::Sync>(
    r: RequestType,
) -> Result<row::Row, Error> {

    match Client::connect(DB_AUTH_STRING, NoTls) {

        Ok(mut client) => match client.query_one(
            r.query(),
            &[&r,]
        ) {
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

