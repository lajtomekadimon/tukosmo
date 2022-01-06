use postgres::{Client, NoTls, Error, row};

use crate::config::global::DB_AUTH_STRING;


pub fn query_db_noparam(
    query_string: &str,
) -> Result<row::Row, Error> {

    match Client::connect(DB_AUTH_STRING, NoTls) {

        Ok(mut client) => client.query_one(query_string, &[]),

        Err(e) => {
            println!("{}", e);  // this is just for fixing bugs
            Err(e)
        },

    }

}

