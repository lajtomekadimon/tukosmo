use actix_web::{App, HttpServer};
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web_middleware_redirect_https::RedirectHTTPS;
use rand::Rng;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

mod config;
use crate::config::global::config as config_data;

mod database;
use crate::database::{
    initdb::initdb,
    query_db_noparam::query_db_noparam,
};

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
    // Configuration
    println!("Loading configuration...");
    let config = config_data();

    // Ports and domain
    let server_mode = config.server.mode;

    let devel_http_port = config.server.development.http_port;
    let devel_https_port = config.server.development.https_port;
    let prod_http_port = config.server.production.http_port;
    let prod_https_port = config.server.production.https_port;
    let domain = config.server.domain;

    let http_port = match server_mode.as_str() {
        "development" => devel_http_port,
        "production" => prod_http_port,
        _ => panic!("Wrong mode."),
    };
    let https_port = match server_mode.as_str() {
        "development" => devel_https_port,
        "production" => prod_https_port,
        _ => panic!("Wrong mode."),
    };

    let http_domain = format!("{}{}", domain, http_port);
    let https_domain = format!("{}{}", domain, https_port);

    // Reset database if users wants to
    let reset_db = config.server.reset;
    if reset_db.as_str() == "true" {
        initdb(&config_data()).unwrap();
    } else if reset_db.as_str() != "false" {
        panic!("Wrong reset value in Tukosmo.toml");
    }

    // Minify CSS and JS
    println!("Minifying CSS code...");
    minify_css(&config.server.theme);
    println!("Minifying JS code...");
    minify_js();
    
    // Delete all previous sessions
    if let Err(e) = query_db_noparam(
        &config_data(),
        "SELECT as_clean_sessions()",
    ) {
        panic!("Database couldn't delete all sessions. Error: {}", e);
    }

    println!("Done!");

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

    println!("Server ready. Visit at: https://{}", https_domain);

    // --------- //

    HttpServer::new(move || {
        App::new()
            .data(config_data())

            .wrap(RedirectHTTPS::with_replacements(
                &[(http_port.to_owned(), https_port.to_owned())]
            ))

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
    .bind(http_domain)?
    .bind_openssl(https_domain, builder)?
    .run()
    .await
}
