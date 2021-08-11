use actix_web::{web, App, HttpServer};
//use actix_web::http;
//use actix_cors::Cors;
use actix_files::Files;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use rand::Rng;

mod config;

mod auth;

mod database;

mod templates;

mod handlers;
use crate::handlers::root::root;
use crate::handlers::login::login;
use crate::handlers::logout::logout;
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

    // AUTH COOKIE
    // -----------
    // Generate a random 32 byte key. Note that it is important to use a
    // unique private key for every project. Anyone with access to the
    // key can generate authentication cookies for any user!
    let private_key = rand::thread_rng().gen::<[u8; 32]>();

    println!("Server ready. Visit at: http://localhost:8080");

    // --------- //

    HttpServer::new(move || {
        App::new()
            /*.wrap(Cors::default()
                // TODO: This has to work with "*" instead of localhost!
                .allowed_origin("http://localhost:8080")

                .allowed_methods(vec!["GET", "POST"])
                .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                .allowed_header(http::header::CONTENT_TYPE)
            )*/
            .wrap(
                IdentityService::new(
                    CookieIdentityPolicy::new(&private_key)
                        .name("auth")
                        .secure(true)
                        .http_only(true)
                        .max_age(604800)  // 1 week
                )
            )

            // Website: /
            .service(root)

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

            // Login
            .service(login)

            // Logout
            .service(logout)

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

                    // Login: /{lang}/admin/login
                    .service(admin::login::login)

                    //-- General --//

                    // Statistics: /{lang}/admin/statistics
                    .service(admin::statistics::statistics)

                    // Server: /{lang}/admin/server
                    .service(admin::server::server)

                    //-- Data --//

                    // Users: /{lang}/admin/users
                    .service(admin::users::users)

                    // Languages: /{lang}/admin/languages
                    .service(admin::languages::languages)

                    // Posts: /{lang}/admin/posts
                    .service(admin::posts::posts)

                    // Pages: /{lang}/admin/pages
                    .service(admin::pages::pages)

                    // Files: /{lang}/admin/files
                    .service(admin::files::files)

                    //-- Settings --//

                    // Website: /{lang}/admin/website
                    .service(admin::website::website)

                    // Tukosmo: /{lang}/admin/tukosmo
                    .service(admin::tukosmo::tukosmo)
                )
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
