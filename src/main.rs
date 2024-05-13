use crate::server::build;

mod server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    build().await
}

