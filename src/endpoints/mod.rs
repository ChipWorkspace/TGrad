use std::path::PathBuf;

use axum::Router;

use tower_http::services::ServeFile;

pub(crate) fn extras_router() -> Router {
    Router::new().route_service(
        "/favicon.ico",
        ServeFile::new(PathBuf::from("assets/favicon.ico")),
    )
}
