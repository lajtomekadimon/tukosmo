use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use postgres_types::{ToSql, FromSql};
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
use crate::templates::admin::dashboard::Dashboard;


pub fn ra_dashboard(
    lang_code: &str,
) -> String {
    "/{lang}/admin".replace("{lang}", lang_code)
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct AgiDashboard {
    pub req: types::AdminRequest,
}

impl QueryFunction for AgiDashboard {
    fn query(&self) -> &str {
        "SELECT aha_g_dashboard($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct AgoDashboard {
    pub data: types::AdminDataDB,
    pub routes: Vec<types::RouteDB>,
    pub visitors_last_month: i64,
}


pub async fn dashboard_get(
    config: web::Data<Config>,
    codename: web::Data<String>,
    req: HttpRequest,
    id: Identity,
) -> impl Responder {

    match user_request(req, id) {

        Ok(user_req) => match query_db(
            &config,
            AgiDashboard {
                req: user_req.clone(),
            },
        ).await {

            Ok(row) => {

                let q: AgoDashboard = row.get(0);
                let t = &t(&q.data.lang.code);

                let sys = System::new();
                let mount = sys.mount_at("/").unwrap();
                let disk_free: i64 = mount.avail.as_u64() as i64;
                let disk_total: i64  = mount.total.as_u64() as i64;
                let disk_used = disk_total - disk_free;
                let disk_used_percentage = (
                    (disk_used as f64) / (disk_total as f64)
                ) * 100.0;
                let disk_used_percent = (f64::trunc(
                    disk_used_percentage * 10.0
                ) / 10.0).to_string();

                let blog_enabled = config.modules.blog.enabled == "yes";
                let gallery_enabled = config.modules.gallery.enabled == "yes";
                let faq_enabled = config.modules.faq.enabled == "yes";
                let payments_enabled =
                    config.modules.payments.enabled == "yes";
                let subscriptions_enabled =
                    config.modules.subscriptions.enabled == "yes";
                let shop_enabled = config.modules.shop.enabled == "yes";

                let html = Dashboard {
                    domain: &config.server.domain,
                    codename: &codename,
                    config: &config,
                    title: t.tukosmo_admin_panel,
                    q: &q,
                    t: t,
                    server_os: &config.server.server_os,
                    disk_used: &disk_used,
                    disk_total: &disk_total,
                    disk_used_percent: &disk_used_percent,
                    blog_enabled: &blog_enabled,
                    gallery_enabled: &gallery_enabled,
                    faq_enabled: &faq_enabled,
                    payments_enabled: &payments_enabled,
                    subscriptions_enabled: &subscriptions_enabled,
                    shop_enabled: &shop_enabled,
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

