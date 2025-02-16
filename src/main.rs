mod middleware;
mod models;
mod routers;

use crate::middleware::logger::LogMiddleware;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting the server at http://127.0.0.1:8080");
    HttpServer::new(move || App::new().wrap(LogMiddleware).configure(routers::config))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
