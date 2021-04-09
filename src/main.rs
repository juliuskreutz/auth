use actix_session::CookieSession;
use actix_web::{App, HttpServer};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rand::Rng;

mod config;
mod database;
mod models;
mod templates;

mod auth;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();

    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    let manager = SqliteConnectionManager::file(config::database());
    let pool = Pool::new(manager).expect("Couldn't create database pool");

    database::init(
        &pool
            .get()
            .expect("Couldn't get database connection from pool"),
    )
    .expect("Couldn't initialize database");

    let secret_key = rand::thread_rng().gen::<[u8; 32]>();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(
                CookieSession::private(&secret_key)
                    .name("auth")
                    .secure(false),
            )
            .configure(auth::config)
            .service(actix_files::Files::new("/static", "static"))
    })
    .bind_openssl(format!("{}:{}", config::local_domain(), config::port()), builder)?
    .run()
    .await
}
