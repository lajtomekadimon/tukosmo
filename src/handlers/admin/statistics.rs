use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use postgres_types::{ToSql, FromSql};

use crate::handlers::admin::user_request::user_request;
use crate::i18n::t::t;
use crate::templates::admin::statistics::Statistics;
use crate::database::types;
use crate::database::query_db::{QueryFunction, query_db};


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct StatisticsARequest {
    pub req: types::AdminRequest,
}

impl QueryFunction for StatisticsARequest {
    fn query(&self) -> &str {
        "SELECT awa_statistics($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct StatisticsAResponse {
    pub data: types::AdminDataDB,
}


pub async fn statistics(
    req: HttpRequest,
    id: Identity,
) -> impl Responder {

    match user_request(req, id) {

        Ok(user_req) => match query_db(
            StatisticsARequest {
                req: user_req,
            },
        ) {

            Ok(row) => {

                let q: StatisticsAResponse = row.get(0);

                let html = Statistics {
                    title: &format!(
                        "{a} - {b}",
                        a = &t("Statistics", &q.data.lang.code),
                        b = &t("Tukosmo Admin Panel", &q.data.lang.code)
                    ),
                    q: &q,
                };

                HttpResponse::Ok().body(html.to_string())

            },

            Err(e) => {
                println!("{}", e);
                HttpResponse::Found()
                    .header("Location", "/")  // TODO
                    .finish()
            },

        },

        Err(redirect_url) => redirect_url,

    }

}

