use actix_web::{web, App, HttpServer};
//use actix_web::http;
//use actix_cors::Cors;
use actix_files::Files;

mod config;

mod auth;

mod database;

mod templates;

mod handlers;
use crate::handlers::website::handler_website;
use crate::handlers::home::handler_home;
use crate::handlers::blog::handler_blog;
use crate::handlers::blog_post::handler_blog_post;
use crate::handlers::page::handler_page;
use crate::handlers::admin;
use crate::handlers::api;

mod minifiers;
use crate::minifiers::css::minify_css;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Here I could do some prerendering, like CSS compression
    println!("Minifying CSS code...");
    minify_css();

    println!("Done!");
    println!("Server ready. Visit at: http://localhost:8080");

    // --------- //

    HttpServer::new(|| {
        App::new()
            /*.wrap(Cors::default()
                // TODO: This has to work with "*" instead of localhost!!!
                .allowed_origin("http://localhost:5000")  // OLD

                .allowed_methods(vec!["GET", "POST"])
                .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                .allowed_header(http::header::CONTENT_TYPE)
            )*/

            // Website: /
            .service(handler_website)

            // API
            .service(web::scope("/api")
                .service(web::scope("/json")
                    // Query: /api/json/query
                    .service(api::json::query::query)

                    // Update: /api/json/update
                    .service(api::json::update::update)

                    // User
                    .service(web::scope("/user")
                        // Login: /api/json/user/login
                        .service(api::json::user::login::login)
                        // Logout: /api/json/user/logout
                        .service(api::json::user::logout::logout)
                        // Signin: /api/json/user/singin
                        .service(api::json::user::signin::signin)
                        // Update: /api/json/user/update
                        .service(api::json::user::update::update),
                    ),
                ),
            )
            
            // Static files: /static/.../...
            .service(Files::new("/static", "static").show_files_listing())

            // HTML pages
            .service(web::scope("/{lang}")
                // Home: /{lang}
                .service(handler_home)

                // Blog: /{lang}/blog
                .service(handler_blog)

                // Blog post: /{lang}/blog/{title}
                .service(handler_blog_post)

                // Page: /{lang}/page/{title}
                .service(handler_page)

                // Admin Panel
                .service(web::scope("/admin")
                    // Dashboard: /{lang}/admin/
                    .service(admin::dashboard::dashboard)

                    /*// General
                    .service(web::scope("/general")
                        // Statistics: /{lang}/admin/general/statistics
                        .service(__)

                        // Server: /{lang}/admin/general/server
                        .service(__)
                    )

                    // Data
                    .service(web::scope("/data")
                        // Users: /{lang}/admin/data/users
                        .service(__)

                        // Posts: /{lang}/admin/data/posts
                        .service(__)

                        // Pages: /{lang}/admin/data/pages
                        .service(__)

                        // Files: /{lang}/admin/data/files
                        .service(__)
                    )

                    // Settings
                    .service(web::scope("/settings")
                        // Website: /{lang}/admin/settings/website
                        .service(__)

                        // Tukosmo: /{lang}/admin/settings/tukosmo
                        .service(__)
                    )*/
                )
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
