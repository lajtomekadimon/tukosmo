use actix_web::{web, App, HttpServer};
//use actix_web::http;
//use actix_cors::Cors;
use actix_files::Files;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use rand::Rng;

mod config;

mod auth;

mod database;
use crate::database::d_all_sessions::d_all_sessions;

mod i18n;

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
use crate::minifiers::minify_css::minify_css;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Here I could do some prerendering, like CSS compression
    println!("Minifying CSS code...");
    minify_css();

    println!("Done!");
    
    // Delete all previous sessions
    if let Err(e) = d_all_sessions() {
        panic!("Database couldn't delete all sessions. Error: {}", e);
    }

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

            // Website root URL: /
            .service(web::resource("/")
                .route(web::get().to(root))
            )

            // API
            .service(web::scope("/api")
                .service(web::scope("/json")
                    // Query
                    .service(web::resource("/query")
                        .route(web::post()
                            .to(api::json::query::query)
                        )
                    )

                    // Update
                    .service(web::resource("/update")
                        .route(web::post()
                            .to(api::json::update::update)
                        )
                    )

                    // User
                    .service(web::scope("/user")
                        // Login
                        .service(web::resource("/login")
                            .route(web::post()
                                .to(api::json::user::login::login)
                            )
                        )

                        // Logout
                        .service(web::resource("/logout")
                            .route(web::post()
                                .to(api::json::user::logout::logout)
                            )
                        )

                        // Sign in
                        .service(web::resource("/signin")
                            .route(web::post()
                                .to(api::json::user::signin::signin)
                            )
                        )

                        // Update
                        .service(web::resource("/update")
                            .route(web::post()
                                .to(api::json::user::update::update)
                            )
                        )
                    ),
                ),
            )
            
            // Static files: /static/.../...
            .service(Files::new(
                // Website route
                "/static",
                // System dir
                "static",
            ).show_files_listing())

            // Login
            .service(web::resource("/login")
                .route(web::post().to(login))
            )

            // Logout
            .service(web::resource("/logout")
                .route(web::get().to(logout))
            )

            // Homepage (/{lang})
            .service(web::resource("/{lang}")
                .route(web::get().to(handler_home))
            )

            // HTML pages
            .service(web::scope("/{lang}")
                // Homepage (/{lang}/)
                .service(web::resource("/")
                    .route(web::get()
                        .to(handler_home)
                    )
                )

                // Blog
                .service(web::resource("/blog")
                    .route(web::get()
                        .to(handler_blog)
                    )
                )

                // Blog post
                .service(web::resource("/blog/{title}")
                    .route(web::get()
                        .to(handler_blog_post)
                    )
                )

                // Page
                .service(web::resource("/page/{title}")
                    .route(web::get()
                        .to(handler_page)
                    )
                )

                // Admin Panel dashboard (/{lang}/admin)
                .service(web::resource("/admin")
                    .route(web::get()
                        .to(admin::dashboard::dashboard)
                    )
                )

                // Admin Panel
                .service(web::scope("/admin")
                    // Dashboard (/{lang}/admin/)
                    .service(web::resource("/")
                        .route(web::get()
                            .to(admin::dashboard::dashboard)
                        )
                    )

                    // Login:
                    .service(web::resource("/login")
                        .route(web::get()
                            .to(admin::login::login)
                        )
                    )

                    //-- General --//

                    // Statistics
                    .service(web::resource("/statistics")
                        .route(web::get()
                            .to(admin::statistics::statistics)
                        )
                    )

                    // Server
                    .service(web::resource("/server")
                        .route(web::get()
                            .to(admin::server::server)
                        )
                    )

                    //-- Data --//

                    // Users
                    .service(web::resource("/users")
                        .route(web::get()
                            .to(admin::users::users)
                        )
                    )

                    // Languages
                    .service(web::resource("/languages")
                        .route(web::get()
                            .to(admin::languages::languages)
                        )
                    )
                    .service(web::resource("/new_language")
                        .route(web::get()
                            .to(admin::new_language::new_language)
                        )
                    )
                    .service(web::resource("/new_language_post")
                        .route(web::post()
                            .to(admin::new_language_post::new_language_post)
                        )
                    )
                    .service(web::resource("/edit_language")
                        .route(web::get()
                            .to(admin::edit_language::edit_language)
                        )
                    )
                    .service(web::resource("/edit_language_post")
                        .route(web::post()
                            .to(admin::edit_language_post::edit_language_post)
                        )
                    )

                    // Posts
                    .service(web::resource("/posts")
                        .route(web::get()
                            .to(admin::posts::posts)
                        )
                    )

                    // Pages
                    .service(web::resource("/pages")
                        .route(web::get()
                            .to(admin::pages::pages)
                        )
                    )

                    // Files
                    .service(web::resource("/files")
                        .route(web::get()
                            .to(admin::files::files)
                        )
                    )

                    //-- Settings --//

                    // Website
                    .service(web::resource("/website")
                        .route(web::get()
                            .to(admin::website::website)
                        )
                    )

                    // Tukosmo
                    .service(web::resource("/tukosmo")
                        .route(web::get()
                            .to(admin::tukosmo::tukosmo)
                        )
                    )
                )
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
