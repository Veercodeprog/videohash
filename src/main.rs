use actix_web::{App, HttpServer};
use videohash::{dhash_api, phash_api}; // Import APIs from your library

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(phash_api).service(dhash_api))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
