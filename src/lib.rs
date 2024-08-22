use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct FormData {
    email: String,
    name: String,
}

async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}

// Let's start simple: we always return a 200 OK
async fn subscribe(_form: web::Form<FormData>) -> impl Responder {
    println!("{}", _form.name);
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> std::io::Result<Server> {
    let server = HttpServer::new(|| {
        //App::new().route("/", web::)
        App::new()
            //.route("/", web::get().to(greet))
            //.route("/{name}", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
