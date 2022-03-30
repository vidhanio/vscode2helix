use std::error::Error;

use actix_web::{post, App, HttpServer, Responder};
use vscode2helix::converter;

#[post("/")]
async fn convert(req_body: String) -> Result<impl Responder, Box<dyn Error>> {
    converter::vscode2helix(&req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(convert))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
