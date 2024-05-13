use actix_web::{App, get, HttpResponse, HttpServer, post, Responder,
                Result, web};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Info {
    app_name: String,
    status: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/info")]
async fn info() -> Result<impl Responder> {
    let info = Info {
        app_name: String::from("actix-1"),
        status: String::from("up"),
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

pub async fn build() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(info)
            .route("hey", web::get().to(manual_hello))
    }).bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use actix_web::{App, http::header::ContentType, test};

    use super::*;

    #[actix_web::test]
    async fn test_hello() {
        let app = test::init_service(App::new().service(hello)).await;
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_request();
        println!("Request: {req:?}");
        let resp = test::call_service(&app, req).await;
        println!("Response: {resp:?}");
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_info() {
        let app = test::init_service(App::new().service(info)).await;
        let req = test::TestRequest::get().uri("/info")
            .insert_header(ContentType::plaintext())
            .to_request();
        println!("Request: {req:?}");
        let resp = test::call_service(&app, req).await;
        println!("Response: {resp:?}");
        assert!(resp.status().is_success());
    }
}
