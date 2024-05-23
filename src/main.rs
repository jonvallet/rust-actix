use actix_web::{App, HttpServer, web};

use crate::server::{echo, hello, info};

mod server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(info)
            .route("hey", web::get().to(crate::server::manual_hello))
    }).bind(("127.0.0.1", 8080))?
        .run()
        .await
}

