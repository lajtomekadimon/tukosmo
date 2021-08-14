use actix_web::{HttpRequest, HttpResponse, Responder};

use crate::i18n::t::t;
use crate::templates::admin::login::Login;


pub async fn login(
    req: HttpRequest,
) -> impl Responder {
    let lang_code: String = req.match_info().get("lang").unwrap().parse().unwrap();

    //let login_route = "/{lang}/admin/login".replace("{lang}", &lang_code);

    let html = Login {
        title: &format!(
            "{a} - {b}",
            a = &t("Login [noun]", &lang_code),
            b = &t("Tukosmo Admin Panel", &lang_code)
        ),
        lang_code: &lang_code,
    };

    HttpResponse::Ok().body(html.to_string())
}

