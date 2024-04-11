use actix_web::{get, web::Path, HttpResponse};

use crate::security;

#[utoipa::path(
    // request_body = None,
    responses(
        (status = 200, description = "Returning the name given as path variable", body = String),
    )
)]
#[get("hello/{name}")]
pub async fn hello(
    name: Path<String>
) -> HttpResponse {
    // Cleaning the string from XSS
    let name_cleaned: String = security::clean_string(name.to_owned());
    let name_input: String = security::untrust_input(name_cleaned);
    HttpResponse::Ok().body(name_input)
}