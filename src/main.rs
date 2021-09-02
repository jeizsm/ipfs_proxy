mod api;
mod auth;
mod env;
mod proxy;
use crate::api::*;
use crate::proxy::proxy;
use actix_cors::Cors;
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use actix_web_httpauth::middleware::HttpAuthentication;
use dotenv;
use env::Config;
use sqlx::postgres::PgPoolOptions;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();
    let config = match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(err) => panic!("{}", err),
    };
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect(&format!(
            "Can't connect to database {}",
            config.database_url
        ));
    HttpServer::new(move || {
        let ipfs_auth = HttpAuthentication::bearer(auth::api_key);
        let api_auth = HttpAuthentication::bearer(auth::jwt);
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method();
        App::new()
            .data(pool.clone())
            .data(config.clone())
            .wrap(cors)
            .wrap(actix_web::middleware::Logger::default())
            .service(
                web::scope("/api/v0")
                    .wrap(ipfs_auth)
                    .default_service(web::route().to(proxy)),
            )
            .service(
                web::scope("/service_api")
                    .route("/sign_up", web::post().to(sign_up))
                    .route("/login", web::post().to(login))
                    .service(
                        web::resource("/api_key")
                            .wrap(api_auth)
                            .route(web::post().to(create_api_key))
                            .route(web::delete().to(disable_api_key)),
                    ),
            )
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind(("127.0.0.1", 5002))?
    .run()
    .await
}
