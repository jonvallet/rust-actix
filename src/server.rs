use actix_web::{get, HttpResponse, post, Responder,
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

pub(crate) async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[cfg(test)]
mod tests {
    use actix_web::{App, http::header::ContentType, test};
    use serde_json::json;

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

        let result: serde_json::Value = test::read_body_json(resp).await;
        println!("Json: {result:?}");

        let expected = Info {
            app_name: String::from("actix-1"),
            status: String::from("up"),
        };
        assert_eq!(result, json!(expected));
    }
}
