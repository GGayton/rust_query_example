use tracing::{info, span, Level};
use sqlx::{postgres::PgPoolOptions, types::time::OffsetDateTime, Pool, Postgres};

use std::time::Instant;

use axum::{
    routing::get,
    Router,
};

use polars::prelude::*;

mod dataframe_generator;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    tracing_subscriber::fmt().init();

    let span = span!(Level::INFO, "program");
    let _guard = span.enter();

    info!("Spinning up...");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://user:admin@localhost/users")
        .await?;

    info!("Connections to database successful");

    // build our application with a single route
    let pool_insert = pool.clone();
    let app = Router::new().route("/", get(move || async { fetch_data(pool_insert).await }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("localhost:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    pool.close().await;

    info!("Exiting");

    Ok(())

}

/// Clon
async fn fetch_data(pool : Pool<Postgres>) -> String {

    let start = Instant::now();

    let result = dataframe_generator::from_postgresql_server(&pool).await;

    match result {
     Ok(df) => String::from(format!("{:#?}", df)),
     Err(err) => String::from(format!("{:#?}", err))
    }
}
