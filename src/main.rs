use actix_web::{App, HttpServer};
//use actix_web::middleware;
//use actix_web::http;
//use actix_cors::Cors;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web_middleware_redirect_https::RedirectHTTPS;
use rand::Rng;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

mod config;

mod database;
use crate::database::query_db_noparam::query_db_noparam;

mod i18n;

mod files;

mod routes;

mod templates;

mod handlers;

mod markdown;

mod minifiers;
use crate::minifiers::{
    minify_css::minify_css,
    minify_js::minify_js,
};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Minifying CSS code...");
    minify_css();
    println!("Minifying JS code...");
    minify_js();

    println!("Done!");
    
    // Delete all previous sessions
    if let Err(e) = query_db_noparam("SELECT as_clean_sessions()") {
        panic!("Database couldn't delete all sessions. Error: {}", e);
    }


    // SSL (HTTPS)
    // -----------
    // Load SSL keys.
    // To create a self-signed temporary cert for testing:
    // openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out \
    // cert.pem -days 365 -subj '/CN=localhost'
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())
        .unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();



    // AUTH COOKIE
    // -----------
    // Generate a random 32 byte key. Note that it is important to use a
    // unique private key for every project. Anyone with access to the
    // key can generate authentication cookies for any user!
    let private_key = rand::thread_rng().gen::<[u8; 32]>();

    println!("Server ready. Visit at: https://localhost:8443");

    // --------- //

    HttpServer::new(move || {
        App::new()
            .wrap(RedirectHTTPS::with_replacements(
                &[(":8080".to_owned(), ":8443".to_owned())]
            ))

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

            // Firefox's view source code doesn't work with this (?)
            //.wrap(middleware::Compress::default())

            // Website root: /
            .service(routes::root::route())

            // Static files: /static/.../...
            .service(routes::staticf::routes())

            // Uploaded files: /files/...
            .service(routes::files::routes())

            // Homepage: /{lang}
            .service(routes::lang::route())
            .service(routes::lang::subroutes())
    })
    .bind("127.0.0.1:8080")?
    .bind_openssl("127.0.0.1:8443", builder)?
    .run()
    .await
}
