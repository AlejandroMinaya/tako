use crate::app::tasks::{Oswald, Task};
use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn start(mut oswald: Oswald) {
    let _ = oswald.load().await;

    let oswald = Arc::new(Mutex::new(oswald));
    let app = Router::new()
        .route("/tasks/", get(get_tasks).post(add_task))
        .with_state(oswald);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:1337").await.unwrap();
    axum::serve(listener, app).await.unwrap()
}

async fn get_tasks(State(oswald): State<Arc<Mutex<Oswald>>>) -> Json<Value> {
    let oswald = oswald.lock().await;
    Json(json!(oswald.get_tasks()))
}

#[axum::debug_handler]
async fn add_task(
    State(oswald): State<Arc<Mutex<Oswald>>>,
    Json(task): Json<Box<Task>>,
) -> Result<StatusCode, StatusCode> {
    let mut oswald = oswald.lock().await;
    oswald.add_task(task);
    let _ = oswald.save().await;
    Ok(StatusCode::CREATED)
}
