use std::{fs::File, io::BufReader};

use serde_json::Value;

pub fn name() -> String {
    match get("name") {
        Value::String(name) => name,
        _ => panic!("Please tell me your name ;)")
    }
}

pub fn domain() -> String {
    match get("domain") {
        Value::String(domain) => domain,
        _ => "localhost".to_owned()
    }
}

pub fn port() -> String {
    match get("port") {
        Value::String(port) => port,
        _ => "8080".to_owned()
    }
}

pub fn database() -> String {
    match get("database") {
        Value::String(db) => db,
        _ => "db.sqlite".to_owned()
    }
}

pub fn smtp_username() -> String {
    match get("smtp-username") {
        Value::String(username) => username,
        _ => panic!("smtp-username not configured")
    }
}

pub fn smtp_password() -> String {
    match get("smtp-password") {
        Value::String(password) => password,
        _ => panic!("smtp-password not configured")
    }
}

pub fn smtp_host() -> String {
    match get("smtp-host") {
        Value::String(host) => host,
        _ => panic!("smtp-host not configured")
    }
}

pub fn salt() -> String {
    match get("salt") {
        Value::String(salt) => salt,
        _ => panic!("salt not configured")
    }
}

fn get(name: &str) -> Value {
    let file = File::open("config.json").expect("Couldn't find config");
    let config: Value = serde_json::from_reader(BufReader::new(file)).expect("Couldn't read config");
    config[name].clone()
}