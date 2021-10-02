use actix_session::CookieSession;
use actix_web::{App, HttpServer};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rand::Rng;
use rustls::{
    internal::pemfile::{certs, pkcs8_private_keys},
    NoClientAuth, ServerConfig,
};

mod config;
mod database;
mod models;
mod templates;

mod auth;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let manager = SqliteConnectionManager::file(config::database());
    let pool = Pool::new(manager).expect("Couldn't create database pool");

    database::init(
        &pool
            .get()
            .expect("Couldn't get database connection from pool"),
    )
    .expect("Couldn't initialize database");

    //let secret_key = rand::thread_rng().gen::<[u8; 32]>();

    let config = load_ssl();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(
                CookieSession::private(&rand::thread_rng().gen::<[u8; 32]>())
                    .name("auth")
                    .secure(false),
            )
            .configure(auth::config)
            .service(actix_files::Files::new("/static", "static"))
    })
    .bind_rustls(
        format!("{}:{}", config::local_domain(), config::port()),
        config,
    )?
    .run()
    .await
}

fn load_ssl() -> ServerConfig {
    use std::{fs::File, io::BufReader};

    let mut config = ServerConfig::new(NoClientAuth::new());
    let cert_file = &mut BufReader::new(File::open("keys/cert.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("keys/key.pem").unwrap());
    let cert_chain = certs(cert_file).unwrap();
    let mut keys = pkcs8_private_keys(key_file).unwrap();
    config.set_single_cert(cert_chain, keys.remove(0)).unwrap();

    config
}
