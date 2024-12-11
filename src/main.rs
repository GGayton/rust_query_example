use tracing::{info, span, Level};

use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {

    tracing_subscriber::fmt().init();

    let span = span!(Level::INFO, "program");
    let _guard = span.enter();

    info!("Spinning up...");
    

    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("localhost:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    info!("Exiting");

}
