use crate::api::ApiKeyDb;
use crate::api::UserDb;
use crate::Config;
use actix_web::dev::ServiceRequest;
use actix_web::web;
use actix_web::HttpMessage;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web_httpauth::extractors::AuthenticationError;
use hmac::{Hmac, NewMac};
use jwt::VerifyWithKey;
use sha2::Sha256;
use sqlx::PgPool;
use std::collections::BTreeMap;
use uuid::Uuid;

pub async fn api_key(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, actix_web::Error> {
    let pool = req.app_data::<web::Data<PgPool>>().unwrap().get_ref();
    let uuid = Uuid::parse_str(credentials.token()).unwrap();
    let key = sqlx::query!(
        "SELECT * FROM api_keys WHERE api_key = $1 and enabled = 'true'",
        uuid
    )
    .fetch_one(pool)
    .await;
    if key.is_ok() {
        let key = sqlx::query!(
            "INSERT INTO logs (api_key_id) VALUES ($1) RETURNING * ",
            key.unwrap().id
        )
        .fetch_one(pool)
        .await;
        Ok(req)
    } else {
        Err(actix_web::web::HttpResponse::Unauthorized().into())
    }
}

pub async fn jwt(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, actix_web::Error> {
    let pool = req.app_data::<web::Data<PgPool>>().unwrap().get_ref();
    let config = req.app_data::<web::Data<Config>>();
    let key: Hmac<Sha256> = Hmac::new_from_slice(config.unwrap().jwt_key.as_bytes()).unwrap();
    let claims: BTreeMap<String, String> = credentials.token().verify_with_key(&key).unwrap();
    let user_id: i64 = claims["id"].parse().unwrap();
    let user = sqlx::query_as!(UserDb, "SELECT * FROM users WHERE id = $1", user_id)
        .fetch_one(pool)
        .await;
    if user.is_ok() {
        req.extensions_mut().insert(user.unwrap().id);
        Ok(req)
    } else {
        Err(actix_web::web::HttpResponse::Unauthorized().into())
    }
}
