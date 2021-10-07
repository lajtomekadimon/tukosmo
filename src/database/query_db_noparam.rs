use postgres::{Client, NoTls, Error, row};

use crate::config::global::db_auth_string;


pub fn query_db_noparam(
    function_name: &str,
) -> Result<row::Row, Error> {

    match Client::connect(db_auth_string(), NoTls) {

        Ok(mut client) => client.query_one(function_name, &[]),

        Err(e) => {
            println!("{}", e);  // this is just for fixing bugs
            Err(e)
        },

    }

}

