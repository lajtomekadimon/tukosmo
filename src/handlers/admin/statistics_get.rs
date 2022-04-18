use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use postgres_types::{ToSql, FromSql};
use chrono::{
    DateTime,
    offset::Utc,
};
use systemstat::{System, Platform};

use crate::config::global::Config;
use crate::handlers::admin::user_request::user_request;
use crate::database::{
    types,
    query_db::{QueryFunction, query_db},
};
use crate::i18n::{
    t::t,
    error_admin_route::error_admin_route,
};
use crate::templates::admin::statistics::Statistics;


pub fn ra_statistics(
    lang_code: &str,
) -> String {
    "/{lang}/admin/statistics".replace("{lang}", lang_code)
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct AgiStatistics {
    pub req: types::AdminRequest,
}

impl QueryFunction for AgiStatistics {
    fn query(&self) -> &str {
        "SELECT aha_g_statistics($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct AgoStatistics {
    pub data: types::AdminDataDB,
    pub routes: Vec<types::RouteDB>,

    /*---*/

    pub visitors_today: i64,
    pub visitors_yesterday: i64,
    pub visitors_last_week: i64,
    pub visitors_last_month: i64,
    pub visitors_last_year: i64,
    pub visitors_total: i64,

    pub visits_today: i64,
    pub visits_yesterday: i64,
    pub visits_last_week: i64,
    pub visits_last_month: i64,
    pub visits_last_year: i64,
    pub visits_total: i64,

    /*---*/

    pub chart_visits_labels: String,
    pub chart_visits_visitors: String,
    pub chart_visits_visits: String,

    pub chart_referrals_labels: String,
    pub chart_referrals_visitors: String,
    pub chart_referrals_visits: String,

    pub chart_browsers_labels: String,
    pub chart_browsers_visitors: String,

    pub chart_platforms_labels: String,
    pub chart_platforms_visitors: String,

    pub chart_countries_labels: String,
    pub chart_countries_visitors: String,

    pub chart_network_labels: String,
    pub chart_network_uploaded: String,
    pub chart_network_downloaded: String,

    pub chart_cpu_labels: String,
    pub chart_cpu_cores: String,

    pub chart_memory_labels: String,
    pub chart_memory_memory: String,
}


pub async fn statistics_get(
    config: web::Data<Config>,
    codename: web::Data<String>,
    start_date: web::Data<DateTime<Utc>>,
    req: HttpRequest,
    id: Identity,
) -> impl Responder {

    match user_request(req, id) {

        Ok(user_req) => match query_db(
            &config,
            AgiStatistics {
                req: user_req.clone(),
            },
        ).await {

            Ok(row) => {

                let q: AgoStatistics = row.get(0);
                let t = &t(&q.data.lang.code);

                let current_date = chrono::offset::Utc::now();
                let old_date: &DateTime<Utc> = &start_date;
                let secs_running = current_date
                    .signed_duration_since(old_date.clone())
                    .num_seconds();

                let sys = System::new();
                let mount = sys.mount_at("/").unwrap();
                let disk_free: i64 = mount.avail.as_u64() as i64;
                let disk_total: i64  = mount.total.as_u64() as i64;
                let disk_used = disk_total - disk_free;

                let html = Statistics {
                    domain: &config.server.domain,
                    codename: &codename,
                    title: &format!(
                        "{a} - {b}",
                        a = t.statistics,
                        b = t.tukosmo_admin_panel,
                    ),
                    q: &q,
                    t: t,
                    start_date: &start_date,
                    secs_running: &secs_running,
                    disk_used: &disk_used,
                    disk_free: &disk_free,
                    disk_total: &disk_total,
                };

                HttpResponse::Ok()
                    .content_type("text/html; charset=UTF-8")
                    .body(html.to_string())

            },

            Err(e) => error_admin_route(&e, &user_req.lang_code),

        },

        Err(redirect_url) => redirect_url,

    }

}

