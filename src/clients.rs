
pub mod api {
    use axum::{
        Router,
        routing::get
    };
    pub async fn start() {
        let app = Router::new()
            .route("/", get(|| async { "Klk manin" }));


        let listener = tokio::net::TcpListener::bind("0.0.0.0:1337").await.unwrap();
        axum::serve(listener, app).await.unwrap()
    }
}
