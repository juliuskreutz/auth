use r2d2::PooledConnection;
use r2d2_sqlite::{
    rusqlite::{params, Result, NO_PARAMS},
    SqliteConnectionManager,
};

use crate::models::{Confirmation, User};

pub type Conn = PooledConnection<SqliteConnectionManager>;

pub fn init(conn: &Conn) -> Result<()> {
    conn.execute(
        "
    CREATE TABLE IF NOT EXISTS users (
        email STRING PRIMARY KEY,
        password STRING
    )",
        NO_PARAMS,
    )?;

    conn.execute(
        "
    CREATE TABLE IF NOT EXISTS confirmations (
        uuid STRING(128) PRIMARY KEY,
        email STRING,
        password STRING
    )",
        NO_PARAMS,
    )?;

    Ok(())
}

pub fn add_user(conn: &Conn, user: &User) -> Result<()> {
    let mut stmt =
        conn.prepare_cached("INSERT OR REPLACE INTO users (email, password) VALUES (?1, ?2)")?;

    stmt.execute(params![user.email(), user.password()])?;

    Ok(())
}

pub fn get_user(conn: &Conn, email: &str) -> Result<Option<User>> {
    let mut stmt = conn.prepare_cached("SELECT email, password FROM users WHERE email = ?1")?;

    let mut rows = stmt.query_map(params! {email}, |row| {
        Ok(User::new(row.get(0)?, row.get(1)?))
    })?;

    if let Some(user) = rows.next() {
        return Ok(Some(user?));
    }

    Ok(None)
}

pub fn add_confirmation(conn: &Conn, confirmation: &Confirmation) -> Result<()> {
    let mut stmt = conn.prepare_cached(
        "INSERT OR REPLACE INTO confirmations (uuid, email, password) VALUES (?1, ?2, ?3)",
    )?;

    stmt.execute(params![
        confirmation.uuid(),
        confirmation.email(),
        confirmation.password()
    ])?;

    Ok(())
}

pub fn get_confirmation(conn: &Conn, uuid: &str) -> Result<Option<Confirmation>> {
    let mut stmt =
        conn.prepare_cached("SELECT uuid, email, password FROM confirmations WHERE uuid = ?1")?;

    let mut rows = stmt.query_map(params! {uuid}, |row| {
        Ok(Confirmation::new(row.get(0)?, row.get(1)?, row.get(2)?))
    })?;

    if let Some(confimation) = rows.next() {
        return Ok(Some(confimation?));
    }

    Ok(None)
}

pub fn delete_confirmation(conn: &Conn, uuid: &str) -> Result<()> {
    let mut stmt = conn.prepare_cached("DELETE FROM confirmations WHERE uuid = ?1")?;

    stmt.execute(params! {uuid})?;

    Ok(())
}
