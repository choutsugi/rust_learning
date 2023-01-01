mod header_user_agent;
mod mirror_body_json;
mod mirror_body_string;
mod path_variables;
mod query_params;
mod root_route;

use axum::{
    body::Body,
    routing::{get, post},
    Router,
};
use header_user_agent::header_user_agent;
use mirror_body_json::mirror_body_json;
use mirror_body_string::mirror_body_string;
use path_variables::path_variables;
use query_params::query_params;

pub fn create_routes() -> Router<Body> {
    Router::new()
        .route("/", get(root_route::hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variables/:id", get(path_variables))
        .route("/query_params", get(query_params))
        .route("/header_user_agent", get(header_user_agent))
}
