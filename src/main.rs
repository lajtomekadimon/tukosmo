use actix_web::{web, http, App, HttpServer};
use actix_cors::Cors;
use actix_files as fs;

mod auth;

mod database;

mod templates;

mod handlers;
use crate::handlers::website::handler_website;
use crate::handlers::admin::handler_admin;
use crate::handlers::api_json_query::handler_api_json_query;
use crate::handlers::api_json_update::handler_api_json_update;
use crate::handlers::api_json_user_login::handler_api_json_user_login;
use crate::handlers::api_json_user_logout::handler_api_json_user_logout;
use crate::handlers::api_json_user_signin::handler_api_json_user_signin;
use crate::handlers::api_json_user_update::handler_api_json_user_update;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::default()
                // TODO: This has to work with "*" instead of localhost!!!
                .allowed_origin("http://localhost:5000")  // for Svelte app development

                .allowed_methods(vec!["GET", "POST"])
                .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                .allowed_header(http::header::CONTENT_TYPE)
            )

            // Website
            .service(handler_website)  // "/"

            // Admin
            .service(handler_admin)  // "/admin"

            // Static files
            .service(fs::Files::new("/static", "static").show_files_listing())

            // API
            .service(web::scope("/api")
                .service(web::scope("/json")
                    .service(handler_api_json_query)  // "/query"
                    .service(handler_api_json_update)  // "/update"
                    .service(web::scope("/user")
                        .service(handler_api_json_user_login)  // "/login"
                        .service(handler_api_json_user_logout)  // "/logout"
                        .service(handler_api_json_user_signin)  // "/signin"
                        .service(handler_api_json_user_update),  // "/update"
                    ),
                ),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
