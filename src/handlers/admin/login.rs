use actix_web::{get, HttpRequest, HttpResponse, Responder};

//use crate::auth::current_session::current_session;
use crate::templates::admin::login::Login;


#[get("/login")]
async fn login(
    req: HttpRequest,
) -> impl Responder {
    let lang_value: String = req.match_info().get("lang").unwrap().parse().unwrap();

    let html = Login {
        title: "Login - Tukosmo Admin Panel",
        lang_code: &lang_value,
    };

    HttpResponse::Ok().body(html.to_string())
}

