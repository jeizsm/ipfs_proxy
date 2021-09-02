use crate::Config;
use actix_web::web;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Responder;
use derive_more::{Display, Error};
use hmac::{Hmac, NewMac};
use jwt::SignWithKey;
use jwt::VerifyWithKey;
use serde::Deserialize;
use serde::Serialize;
use sha2::Sha256;
use sqlx::postgres::PgPool;
use std::collections::BTreeMap;
use uuid::Uuid;

#[derive(Debug, Display, Error)]
pub enum Error {
    #[display(fmt = "internal error {:?}", _0)]
    SqlxError(sqlx::Error),
    #[display(fmt = "internal error {:?}", _0)]
    JwtError(jwt::Error),
    #[display(fmt = "internal error")]
    HmacError,
}

impl actix_web::error::ResponseError for Error {}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserDb {
    pub id: i64,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiKey {
    pub api_key: Uuid,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiKeyDb {
    pub id: i64,
    pub user_id: i64,
    pub api_key: Uuid,
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JwtKey {
    pub key: String,
}

#[derive(Serialize, Debug)]
pub struct LogsDb {
    pub id: i64,
    pub api_key_id: i64,
    pub time: time::PrimitiveDateTime,
}

pub async fn create_api_key(
    pool: web::Data<PgPool>,
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    let extensions = req.extensions();
    let user_id = extensions.get::<i64>().unwrap();
    let api_key = Uuid::new_v4();
    let conn = pool.get_ref();
    let key = sqlx::query_as!(
        ApiKeyDb,
        "INSERT INTO api_keys(api_key, user_id) VALUES ($1, $2) RETURNING *",
        api_key,
        user_id
    )
    .fetch_one(conn)
    .await
    .map_err(|err| Error::SqlxError(err))?;
    Ok(HttpResponse::Ok().json(key))
}

pub async fn disable_api_key(
    api_key: web::Json<ApiKey>,
    pool: web::Data<PgPool>,
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    let extensions = req.extensions();
    let user_id = extensions.get::<i64>().unwrap();
    let conn = pool.get_ref();
    let key = sqlx::query_as!(
        ApiKeyDb,
        "UPDATE api_keys SET enabled = 'false' WHERE api_key = $1 AND user_id = $2 RETURNING *",
        api_key.api_key,
        user_id
    )
    .fetch_one(conn)
    .await
    .map_err(|err| Error::SqlxError(err))?;
    Ok(HttpResponse::Ok().json(key))
}

pub async fn list_api_logs(
    req: HttpRequest,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let extensions = req.extensions();
    let user_id = extensions.get::<i64>().unwrap();
    let conn = pool.get_ref();
    let logs = sqlx::query_as!(
        LogsDb,
        "SELECT logs.id, logs.time, logs.api_key_id FROM logs INNER JOIN api_keys ON logs.api_key_id = api_keys.id WHERE api_keys.user_id = $1",
        user_id
    )
    .fetch_all(conn)
    .await
    .map_err(|err| Error::SqlxError(err))?;
    Ok(HttpResponse::Ok().json(logs))
}

pub async fn sign_up(
    user: web::Json<User>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let conn = pool.get_ref();
    let user = sqlx::query_as!(
        UserDb,
        "INSERT INTO users(username, password) VALUES ($1, $2) RETURNING *",
        user.username,
        user.password
    )
    .fetch_one(conn)
    .await
    .map_err(|err| Error::SqlxError(err))?;
    println!("{:?}", user);
    Ok(HttpResponse::Ok().json(user))
}

pub async fn login(
    user: web::Json<User>,
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
) -> Result<HttpResponse, actix_web::Error> {
    let conn = pool.get_ref();
    let user = sqlx::query_as!(
        UserDb,
        "SELECT * FROM users WHERE username = $1 AND password = $2",
        user.username,
        user.password
    )
    .fetch_one(conn)
    .await
    .map_err(|err| Error::SqlxError(err))?;
    let key: Hmac<Sha256> = Hmac::new_from_slice(&config.get_ref().jwt_key.as_bytes())
        .map_err(|err| Error::HmacError)?;
    let mut claims = BTreeMap::new();
    claims.insert("username", user.username);
    claims.insert("id", user.id.to_string());
    let token_str = claims
        .sign_with_key(&key)
        .map_err(|err| Error::JwtError(err))?;
    Ok(HttpResponse::Ok().json(JwtKey { key: token_str }))
}
