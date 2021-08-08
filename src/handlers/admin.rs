use actix_web::{get, HttpRequest, HttpResponse, Responder};

use crate::templates::admin::dashboard::Dashboard;


// TODO
#[get("/admin")]
async fn handler_admin(
    req: HttpRequest,
) -> impl Responder {
    let lang_value: String = req.match_info().get("lang").unwrap().parse().unwrap();

    let html = Dashboard {
        title: "Tukosmo Admin Panel",
        lang_code: &lang_value,
    };

    HttpResponse::Ok().body(html.to_string())
}

