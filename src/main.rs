mod router;

#[tokio::main]
async fn main() {
    let app = router::get_routes();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:1337").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
