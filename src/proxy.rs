use actix_web::client::{Client, ClientResponse, SendRequestError};
use actix_web::{dev, web, HttpRequest, HttpResponse};

// trait response converter between client and server responses
pub trait IntoHttpResponse {
    fn into_http_response(self) -> HttpResponse;

    fn into_wrapped_http_response<E>(self) -> Result<HttpResponse, E>
    where
        Self: Sized,
    {
        Ok(self.into_http_response())
    }
}

impl IntoHttpResponse for ClientResponse<dev::Decompress<dev::Payload>> {
    fn into_http_response(self) -> HttpResponse {
        let mut response = HttpResponse::build(self.status());

        self.headers().iter().for_each(|(k, v)| {
            response.set_header(k, v.clone());
        });
        response.streaming(self)
    }
}

// Streaming proxy to ipfs
pub async fn proxy(
    req: HttpRequest,
    body: web::Payload,
) -> actix_web::Result<HttpResponse, SendRequestError> {
    let ipfs_host = "localhost:5001";
    let new_uri = format!("http://{}{}", ipfs_host, req.uri());
    Client::default()
        .request_from(&new_uri, req.head())
        .set_header("host", ipfs_host)
        .send_stream(body)
        .await?
        .into_wrapped_http_response()
}
