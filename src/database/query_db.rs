use postgres::{Client, NoTls, Error, row};
use postgres_types::ToSql;

use crate::config::global::db_auth_string;


pub trait QueryFunction {
    fn query(&self) -> &str;
}


pub fn query_db<RequestType: QueryFunction + ToSql + std::marker::Sync>(
    r: RequestType,
) -> Result<row::Row, Error> {

    match Client::connect(db_auth_string(), NoTls) {

        Ok(mut client) => client.query_one(
            r.query(),
            &[&r,]
        ),

        Err(e) => {
            println!("{}", e);  // this is just for fixing bugs
            Err(e)
        },

    }

}

