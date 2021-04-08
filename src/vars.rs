use dotenv::dotenv;

use std::env::var;

pub fn domain() -> String {
    dotenv().ok();

    var("DOMAIN").unwrap_or_else(|_| "localhost".to_owned())
}

pub fn port() -> String {
    dotenv().ok();

    var("PORT").unwrap_or_else(|_| "8080".to_owned())
}

pub fn database() -> String {
    dotenv().ok();

    var("DATABASE").unwrap_or_else(|_| "db.sqlite".to_owned())
}

pub fn smtp_username() -> String {
    dotenv().ok();

    var("SMTP_USERNAME").expect("SMTP_USERNAME is not set")
}

pub fn smtp_password() -> String {
    dotenv().ok();

    var("SMTP_PASSWORD").expect("SMTP_PASSWORD is not set")
}

pub fn smtp_host() -> String {
    dotenv().ok();

    var("SMTP_HOST").expect("SMTP_HOST is not set")
}

pub fn salt() -> String {
    dotenv().ok();

    var("SALT").expect("SALT is not set")
}
