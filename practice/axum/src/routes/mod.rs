mod mirror_body_json;
mod mirror_body_string;

mod root_route;

use axum::{
    body::Body,
    routing::{get, post},
    Router,
};

use mirror_body_json::mirror_body_json;
use mirror_body_string::mirror_body_string;

pub fn create_routes() -> Router<Body> {
    Router::new()
        .route("/", get(root_route::hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
}
