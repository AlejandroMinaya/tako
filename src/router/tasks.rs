use axum::{
    Router,
    routing::get
};

pub fn get_routes() -> Router {
    return Router::new()
    .route("/", get(|| async { "example task\n" }));
}
