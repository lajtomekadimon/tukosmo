use actix_web::{web, App, HttpServer};
//use actix_web::http;
//use actix_cors::Cors;
use actix_files::Files;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use rand::Rng;

mod config;

mod database;
use crate::database::query_db_noparam::query_db_noparam;

mod i18n;

mod templates;

mod handlers;
use crate::handlers::root::root;
use crate::handlers::website;
use crate::handlers::admin;

mod markdown;

mod minifiers;
use crate::minifiers::minify_css::minify_css;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Here I could do some prerendering, like CSS compression
    println!("Minifying CSS code...");
    minify_css();

    println!("Done!");
    
    // Delete all previous sessions
    if let Err(e) = query_db_noparam("SELECT as_clean_sessions()") {
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
                .allowed_headers(
                    vec![http::header::AUTHORIZATION, http::header::ACCEPT]
                )
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

            // Static files: /static/.../...
            .service(Files::new(
                // Website route
                "/static",
                // System dir
                "static",
            ).show_files_listing())


            // Homepage (/{lang})
            .service(web::resource("/{lang}")
                .route(web::get().to(website::home::home))
            )

            // HTML pages
            .service(web::scope("/{lang}")
                // Homepage (/{lang}/)
                .service(web::resource("/")
                    .route(web::get()
                        .to(website::home::home)
                    )
                )

                // Blog (/blog)
                .service(web::resource("/blog")
                    .route(web::get()
                        .to(website::blog::blog)
                    )
                )

                // Blog
                .service(web::scope("/blog")
                    // Blog (/blog/)
                    .service(web::resource("/")
                        .route(web::get()
                            .to(website::blog::blog)
                        )
                    )

                    // Blog post
                    .service(web::resource("/{permalink}")
                        .route(web::get()
                            .to(website::blog_post::blog_post)
                        )
                    )
                )

                // Page
                .service(web::resource("/page/{permalink}")
                    .route(web::get()
                        .to(website::page::page)
                    )
                )

                // Error
                .service(web::resource("/error")
                    .route(web::get()
                        .to(website::error::error)
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

                    // Error
                    .service(web::resource("/error")
                        .route(web::get()
                            .to(admin::error::error)
                        )
                    )

                    // Login
                    .service(web::resource("/login")
                        .route(web::get()
                            .to(admin::login::login)
                        )
                        .route(web::post()
                            .to(admin::login_post::login_post)
                        )
                    )

                    // Logout
                    .service(web::resource("/logout")
                        .route(web::get()
                            .to(admin::logout::logout)
                        )
                    )

                    // Account
                    .service(web::resource("/account")
                        .route(web::get()
                            .to(admin::account::account)
                        )
                    )

                    // Sessions
                    .service(web::resource("/sessions")
                        .route(web::get()
                            .to(admin::sessions::sessions)
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
                    .service(web::resource("/new_user")
                        .route(web::get()
                            .to(admin::new_user::new_user)
                        )
                        .route(web::post()
                            .to(admin::new_user_post::new_user_post)
                        )
                    )
                    .service(web::resource("/edit_user")
                        .route(web::get()
                            .to(admin::edit_user::edit_user)
                        )
                        .route(web::post()
                            .to(admin::edit_user_post::edit_user_post)
                        )
                    )
                    .service(web::resource("/delete_user")
                        .route(web::get()
                            .to(admin::delete_user::delete_user)
                        )
                        .route(web::post()
                            .to(admin::delete_user_post::delete_user_post)
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
                        .route(web::post()
                            .to(admin::new_language_post::new_language_post)
                        )
                    )
                    .service(web::resource("/edit_language")
                        .route(web::get()
                            .to(admin::edit_language::edit_language)
                        )
                        .route(web::post()
                            .to(admin::edit_language_post::edit_language_post)
                        )
                    )
                    .service(web::resource("/delete_language")
                        .route(web::get()
                            .to(admin::delete_language::delete_language)
                        )
                        .route(web::post()
                            .to(admin::delete_language_post::delete_language_post)
                        )
                    )

                    // Posts
                    .service(web::resource("/posts")
                        .route(web::get()
                            .to(admin::posts::posts)
                        )
                    )
                    .service(web::resource("/new_post")
                        .route(web::get()
                            .to(admin::new_post::new_post)
                        )
                        .route(web::post()
                            .to(admin::new_post_post::new_post_post)
                        )
                    )
                    .service(web::resource("/edit_post")
                        .route(web::get()
                            .to(admin::edit_post::edit_post)
                        )
                        .route(web::post()
                            .to(admin::edit_post_post::edit_post_post)
                        )
                    )
                    .service(web::resource("/delete_post")
                        .route(web::get()
                            .to(admin::delete_post::delete_post)
                        )
                        .route(web::post()
                            .to(admin::delete_post_post::delete_post_post)
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
