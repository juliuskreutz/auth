use actix_session::CookieSession;
use actix_web::{App, HttpServer};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rand::Rng;

mod database;
mod models;
mod templates;
mod vars;

mod auth;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let manager = SqliteConnectionManager::file(vars::database());
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
    .bind(format!("{}:{}", vars::domain(), vars::port()))?
    .run()
    .await
}
