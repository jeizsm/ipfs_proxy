use actix_web::{body, client::Client, middleware, web, App, HttpRequest, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn proxy(req: HttpRequest, mut body: web::Bytes) -> impl Responder {
    let new_uri = format!("http://localhost:5001{}", req.path());
    let result = Client::default()
        .request_from(&new_uri, req.head())
        .set_header("host", "localhost:5001")
        .send_body(body)
        .await;
    result.unwrap().body().await
    //format!("{:?}, {:?}, {}", result, req.head(), &new_uri)
    //.send_body(req.to_owned);
    // format!("{}", req.path())
    //"hello ipfs\n"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::NormalizePath::default())
            .service(web::scope("/api/v0").default_service(web::route().to(proxy)))
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind(("127.0.0.1", 5002))?
    .run()
    .await
}
