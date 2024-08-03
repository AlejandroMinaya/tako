
pub mod api {
    use crate::core::tasks::{Oswald, Task};
    use axum::{
        Router,
        extract::State,
        routing::get,
        http::StatusCode,
        Json
    };
    use std::sync::{Arc, Mutex};
    use serde_json::{Value, json};


    pub async fn start(oswald: Oswald) {
        let oswald = Arc::new(Mutex::new(oswald));
        let app = Router::new()
            .route("/tasks/", get(get_tasks).post(add_task))
            .with_state(oswald);


        let listener = tokio::net::TcpListener::bind("0.0.0.0:1337").await.unwrap();
        axum::serve(listener, app).await.unwrap()
    }

    async fn get_tasks(State(oswald): State<Arc<Mutex<Oswald>>>) -> Json<Value>{
        match oswald.lock() {
            Ok(oswald) => Json(json!(oswald.get_tasks())),
            Err(err) => Json(json!(err.to_string()))
        }
    }

    #[axum::debug_handler]
    async fn add_task(State(oswald): State<Arc<Mutex<Oswald>>>, Json(task): Json<Box<Task>>) -> Result<StatusCode, StatusCode> {
        match oswald.lock() {
            Ok(mut oswald) => {
                oswald.add_task(task);
                Ok(StatusCode::CREATED)
            },
            Err(err) => {
                println!("{err}");
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }

}
