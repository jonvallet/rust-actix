use actix_web::{get, post, web, App, HttpResponse, HttpServer, 
    Responder, Result};
use serde::Serialize;

#[derive(Serialize)]
struct Info {
    app_name: String,
    status: String
}


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/info")]
async fn info() -> Result<impl Responder> {
    let info = Info {
        app_name: String::from("actix-1"),
        status: String::from("up")
    };
    Ok(web::Json(info))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(info)
            .route("hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

