use actix_web::{web, App, HttpServer, dev};
use actix_identity::{CookieIdentityPolicy, IdentityService};
use openssl::{
    ssl::{SslAcceptor, SslFiletype, SslMethod},
    pkey::PKey,
    x509::X509,
    asn1::{Asn1Time, Asn1TimeRef},
    error::ErrorStack,
};
use std::time::{
    Duration,
    SystemTime,
};
use std::fs;
use chrono::Utc;
use tokio;
use std::sync::{Arc, Mutex};
use core::future::Future;

use crate::config::{
    global::config as config_data,
    change_new_domain::change_new_domain,
};
use crate::minifiers::{
    minify_css::minify_css,
    minify_js::minify_js,
};
use crate::database::{
    initdb::initdb,
    query_db_noparam::query_db_noparam,
};
use crate::server::{
    gen_tls_cert::gen_tls_cert,
    redirect_https::RedirectHTTPS,
};
use crate::routes;


fn asn1_time_to_system_time(
    time: &Asn1TimeRef,
) -> Result<SystemTime, ErrorStack> {
    let unix_time = Asn1Time::from_unix(0)?.diff(time)?;
    Ok(
        SystemTime::UNIX_EPOCH + Duration::from_secs(
            unix_time.days as u64 * 86400 + unix_time.secs as u64
        )
    )
}


#[derive(Clone)]
pub struct Handle(pub Arc<Mutex<Option<dev::ServerHandle>>>);

impl Handle {
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(None)))
    }

    pub async fn replace(
        &self,
        handle: dev::ServerHandle,
    ) -> Option<dev::ServerHandle> {
        self.0.lock().unwrap().replace(handle)
    }

    pub async fn stop(
        &self,
        graceful: bool,
    ) -> impl Future<Output = ()> {
        self.0.lock().unwrap().take().unwrap().stop(graceful)
    }
}


pub async fn new_server(
    handle: Handle,
    private_key: [u8; 32],
    start_n: i64,
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
        initdb(&config_data()).await.unwrap();
    } else if reset_db.as_str() != "false" {
        panic!("Wrong reset value in Tukosmo.toml");
    }

    // Codename for static files efficient cache policy
    let codename = format!("{:x}", Utc::now().timestamp());

    // Minify CSS and JS
    let bundles_path = "./static/bundles";
    match fs::remove_dir_all(bundles_path) {
        // TODO: (?)
        Ok(_) => {
            fs::create_dir(bundles_path).unwrap();
        },
        Err(_) => {
            fs::create_dir(bundles_path).unwrap();
        },
    }
    minify_css(&config.server.theme);
    minify_js();
    
    // If this is the first server start...
    if start_n == 1 {
        // Delete all previous sessions
        if let Err(e) = query_db_noparam(
            &config_data(),
            "SELECT as_clean_sessions()",
        ).await {
            panic!("Database couldn't delete all sessions. Error: {}", e);
        }
    }

    // SSL (HTTPS)
    // -----------
    let mut ssl_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())
        .unwrap();
    if &server_mode == "production" {
        // Generate or renewal certificate if necessary
        let need_cert = match fs::read("etc/cert.der") {
            Ok(cert_bytes) => {
                let cert_der = X509::from_der(&cert_bytes).unwrap();

                let expiry_date = asn1_time_to_system_time(
                    cert_der.not_after()
                ).unwrap();

                match expiry_date.elapsed() {
                    Ok(_) => true,
                    Err(expiry) => expiry.duration() < Duration::from_secs(
                        // Remember:
                        // - Let's Encrypt certificates last for 90 days.
                        // - They send an expiration notice 20 days before
                        //   the TLS certificate expires.
                        // - We will renew them 30 days before,
                        //   so there will be no expiration notice.
                        30 * 24 * 60 * 60  // 30 days
                    ),
                }
            },
            Err(_) => true,
        };
        if &config.server.new_domain == "true" {
            gen_tls_cert(&user_email, &domain).await.unwrap();
            change_new_domain(
                &config_data(),
                "false"
            );
        } else if need_cert {
            gen_tls_cert(&user_email, &domain).await.unwrap();
        }

        // Add private key
        let pkey_bytes = fs::read("etc/pkey.der").unwrap();
        let pkey_der = PKey::private_key_from_der(
            &pkey_bytes
        ).unwrap();
        ssl_builder.set_private_key(&pkey_der).unwrap();

        // Add certificate
        let cert_bytes = fs::read("etc/cert.der").unwrap();
        let cert_der = X509::from_der(
            &cert_bytes
        ).unwrap();
        ssl_builder.set_certificate(&cert_der).unwrap();

        // Add intermediate certificate to the chain
        let icert_bytes = fs::read("etc/icert.der").unwrap();
        let icert_der = X509::from_der(&icert_bytes).unwrap();
        ssl_builder.add_extra_chain_cert(icert_der).unwrap();

        // Generate thread that restarts server to renew TLS
        let secs_until_expiry = match asn1_time_to_system_time(
            cert_der.not_after()
        ).unwrap().elapsed() {
            Ok(_) => {
                panic!("Certificate expiry date is not valid.");
            },
            Err(expiry) => expiry.duration().as_secs(),
        };
        let handle_for_renew = handle.clone();
        tokio::spawn(async move {
            if secs_until_expiry > (30 * 24 * 60 * 60) {
                // Wait two months
                tokio::time::sleep(
                    Duration::from_secs(
                        // Remember:
                        // - Let's Encrypt certificates last for 90 days.
                        // - They send an expiration notice 20 days before
                        //   the TLS certificate expires.
                        // - We will renew them 30 days before,
                        //   so there will be no expiration notice.
                        secs_until_expiry - (30 * 24 * 60 * 60)
                    )
                ).await;
            }

            // Restart server (to generate a new TLS certificate)
            let _ = handle_for_renew.stop(true);
        });
    } else if &server_mode == "development" {
        // Load TLS private key and certificate (created by make in local)
        ssl_builder
            .set_private_key_file("etc/pkey.pem", SslFiletype::PEM)
            .unwrap();
        ssl_builder.set_certificate_chain_file("etc/cert.pem").unwrap();
    } else {
        panic!("Wrong mode in Tukosmo.toml");
    }

    let private_cookie_key: [u8; 32] = private_key;

    if &server_mode == "development" {
        println!("Server ready. Visit at: https://{}", https_domain);
    } else {
        println!("Server ready. Visit at: https://{}", domain);
    }

    // Start server as normal but don't .await after .run() yet
    let srv = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config_data()))  // Tukosmo.toml values
            .app_data(web::Data::new(codename.clone()))  // cached files code
            .app_data(web::Data::new(handle.clone()))  // to stop server

            .wrap(
                RedirectHTTPS::with_replacements(
                    &[(http_port.to_owned(), https_port.to_owned())]
                )
            )

            .wrap(
                IdentityService::new(
                    CookieIdentityPolicy::new(&private_cookie_key)
                        .name("auth")
                        .secure(true)
                        .http_only(true)
                        .max_age_secs(604800)  // 1 week
                )
            )

            // TODO: .wrap()  // compression

            // Website root: /
            .service(routes::root::route())

            // Static files: /static/.../...
            .service(routes::staticf::route())

            // Uploaded files: /files/...
            .service(routes::files::route())

            // Homepage: /{lang}
            .service(routes::lang::route())
            .service(routes::lang::subroutes())

            // NOT FOUND
            /*
            .default_service(
                web::route()
                    .guard(guard::Not(guard::Get()))
                    .to(|| HttpResponse::MethodNotAllowed()),
            )
            */
    })
    .bind(http_domain).unwrap()
    .bind_openssl(https_domain, ssl_builder).unwrap()
    .shutdown_timeout(0)  // seconds to shutdown after stop signal
                          // TODO: Set value in etc/Tukosmo.toml
    .run();

    srv
}
