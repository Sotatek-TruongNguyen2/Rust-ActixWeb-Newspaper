pub mod configuration;
pub mod routes;
pub mod startup;
pub mod telemetry;

use std::net::TcpListener;

use actix_web::{dev::Server, middleware::Logger, web, App, HttpServer};
use routes::{health_check, subscribe};
use sqlx::PgPool;

pub fn run(listener: TcpListener, db_pool: PgPool) -> std::io::Result<Server> {
    // Wrap the connection in a smart pointer
    let connection = web::Data::new(db_pool);

    let server = HttpServer::new(move || {
        //App::new().route("/", web::)
        App::new()
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
