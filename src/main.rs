use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    // Build our application with a route
    let app = Router::new().route("/", get(handler));

    // Run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}

// Handler for GET /
async fn handler() -> &'static str {
    "Hello, World!"
}