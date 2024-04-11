use actix_web::{App, HttpServer};
use config::ssl::CertificatePair;
use controller::hello_controller;
use doc::openapi::get_openapi;
use utoipa_swagger_ui::SwaggerUi;

mod doc;
mod models;
mod repository;
mod services;
mod config;
mod controller;
mod security;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let certificate_pair = CertificatePair::new("certs/cert.pem", "certs/key.pem");
    
    HttpServer::new(|| {
        App::new()
            .service(hello_controller::hello)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", get_openapi()),
            )
    })
    .bind_rustls_0_22(("0.0.0.0", 8443), certificate_pair.get_server_config())?
    .run()
    .await
}