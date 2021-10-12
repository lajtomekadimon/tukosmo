use postgres::{Client, NoTls, Error, row};
use postgres_types::ToSql;

use crate::config::global::DB_AUTH_STRING;


pub trait QueryFunction {
    fn query(&self) -> &str;
}


pub fn query_db<RequestType: QueryFunction + ToSql + std::marker::Sync>(
    r: RequestType,
) -> Result<row::Row, Error> {

    match Client::connect(DB_AUTH_STRING, NoTls) {

        Ok(mut client) => client.query_one(
            r.query(),
            &[&r,]
        ),

        Err(e) => Err(e),

    }

}

