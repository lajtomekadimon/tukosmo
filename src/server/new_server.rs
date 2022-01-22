use actix_web::{App, HttpServer, dev};
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web_middleware_redirect_https::RedirectHTTPS;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::sync::mpsc;

use crate::config::global::config as config_data;
use crate::minifiers::{
    minify_css::minify_css,
    minify_js::minify_js,
};
use crate::database::{
    initdb::initdb,
    query_db_noparam::query_db_noparam,
};
use crate::routes;


pub fn new_server(
    tx: mpsc::Sender<()>,
    private_key: &[u8; 32],
    start_n: &i64,
) -> dev::Server {
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

    // Reset database if user wants to
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
    
    // If this is the first server start...
    if start_n == &1 {
        // Delete all previous sessions
        if let Err(e) = query_db_noparam(
            &config_data(),
            "SELECT as_clean_sessions()",
        ) {
            panic!("Database couldn't delete all sessions. Error: {}", e);
        }
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

    let private_cookie_key: [u8; 32] = *private_key;

    println!("Server ready. Visit at: https://{}", https_domain);

    // Start server as normal but don't .await after .run() yet
    let server = HttpServer::new(move || {
        App::new()
            .data(config_data())  // to load Tukosmo.toml config values
            .data(tx.clone())  // to stop server

            .wrap(RedirectHTTPS::with_replacements(
                &[(http_port.to_owned(), https_port.to_owned())]
            ))

            .wrap(
                IdentityService::new(
                    CookieIdentityPolicy::new(&private_cookie_key)
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
    .bind(http_domain).unwrap()
    .bind_openssl(https_domain, builder).unwrap()
    .shutdown_timeout(1)  // seconds to shutdown after stop signal
    .run();

    server
}
