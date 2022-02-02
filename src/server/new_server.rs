use actix_web::{App, HttpServer, dev};
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web_middleware_redirect_https::RedirectHTTPS;
use openssl::{
    ssl::{SslAcceptor, SslFiletype, SslMethod},
    pkey::PKey,
    x509::X509,
};
use std::sync::mpsc;
use std::time::Duration;
use std::thread;
use std::convert::TryFrom;

use crate::config::global::config as config_data;
use crate::minifiers::{
    minify_css::minify_css,
    minify_js::minify_js,
};
use crate::database::{
    initdb::initdb,
    query_db_noparam::query_db_noparam,
};
use crate::server::gen_tls_cert::gen_tls_cert;
use crate::routes;


pub fn new_server(
    tx: mpsc::Sender<()>,
    private_key: &[u8; 32],
    start_n: &i64,
) -> dev::Server {
    // Configuration
    let config = config_data();

    // Ports and domain
    let server_mode = config.server.mode;

    let devel_http_port = config.server.development.http_port;
    let devel_https_port = config.server.development.https_port;
    let prod_http_port = config.server.production.http_port;
    let prod_https_port = config.server.production.https_port;
    let domain = config.server.domain;
    let user_email = config.server.user_email;

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
    minify_css(&config.server.theme);
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

    // SSL (HTTPS)
    // -----------
    // TODO: Generate certificates only when it's really needed, and not
    // every time we restart the server.
    let mut ssl_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())
        .unwrap();
    if &server_mode == "production" {
        match gen_tls_cert(
            &user_email,
            &domain,
        ) {
            Ok(cert) => {
                // Generate thread that restarts server to renew TLS
                let cert_valid_days_left = u64::try_from(
                    cert.valid_days_left().unwrap()
                ).ok().unwrap();
                let restarter = tx.clone();
                thread::spawn(move || {
                    // Wait two months
                    thread::sleep(
                        Duration::from_secs(
                            // Remember:
                            // - Let's Encrypt certificates last for 90 days.
                            // - They send an expiration notice 20 days before
                            //   the TLS certificate expires.
                            // - We will renew them 30 days before,
                            //   so there will be no expiration notice.
                            (cert_valid_days_left - 30) * 24 * 60 * 60,
                        )
                    );

                    // Restart server to generate a new TLS certificate
                    restarter.send(()).unwrap();
                });

                let pkey_der = PKey::private_key_from_der(
                    &cert.private_key_der().unwrap()
                ).unwrap();
                ssl_builder.set_private_key(&pkey_der).unwrap();

                let cert_der = X509::from_der(
                    &cert.certificate_der().unwrap()
                ).unwrap();
                ssl_builder.set_certificate(&cert_der).unwrap();
                //ssl_builder.add_extra_chain_cert(cert_der).unwrap();
            },
            Err(e) => {
                panic!("ERROR (TLS): {}", e);
            },
        }
    } else if &server_mode == "development" {
        // Load TLS private key and certificate (created by make in local)
        ssl_builder
            .set_private_key_file("key.pem", SslFiletype::PEM)
            .unwrap();
        ssl_builder.set_certificate_chain_file("cert.pem").unwrap();
    } else {
        panic!("Wrong mode in Tukosmo.toml");
    }

    let private_cookie_key: [u8; 32] = *private_key;

    if &server_mode == "development" {
        println!("Server ready. Visit at: https://{}", https_domain);
    } else {
        println!("Server ready. Visit at: https://{}", domain);
    }

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
    .bind_openssl(https_domain, ssl_builder).unwrap()
    .shutdown_timeout(0)  // seconds to shutdown after stop signal
    .run();

    server
}
