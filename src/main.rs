use actix_web::{middleware, App, HttpServer};

use crate::api::health_check::health;
use crate::services::authorization::check_authorization_header;
use crate::services::sequential_id::Sandflake;

mod api;
mod services;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let sandflake = Sandflake::default(1);

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap_fn(check_authorization_header)
            .service(health)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
