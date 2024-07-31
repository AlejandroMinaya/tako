
pub mod api {
    use crate::core::tasks::Oswald;
    use axum::{
        Router,
        extract::State,
        routing::get,
        Json
    };
    use std::sync::Arc;
    use serde_json::{Value, json};


    pub async fn start(oswald: Oswald) {
        let oswald = Arc::new(oswald);
        let app = Router::new()
            .route("/tasks/", get(get_tasks))
            .with_state(oswald);


        let listener = tokio::net::TcpListener::bind("0.0.0.0:1337").await.unwrap();
        axum::serve(listener, app).await.unwrap()
    }

    async fn get_tasks<'a>(State(oswald): State<Arc<Oswald>>) -> Json<Value>{
        Json(json!(oswald.get_all_tasks()))
    }
}
