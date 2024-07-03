use axum::{
    Router,
};

// Routes Imports
mod tasks;

pub fn get_routes() -> Router {
    return Router::new()
    .nest("/tasks", tasks::get_routes());
}
