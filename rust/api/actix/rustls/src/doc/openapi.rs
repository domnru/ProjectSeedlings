use utoipa::{openapi::OpenApi as OpenApi_F, OpenApi};
use crate::controller::hello_controller;

#[derive(OpenApi)]
#[openapi(
    paths(
        hello_controller::hello
    ),
    components(
        schemas()
    )
)]
pub struct ApiDoc;

pub fn get_openapi() -> OpenApi_F {
    ApiDoc::openapi()
}