use tracing::{info, span, Level};
use sqlx::{postgres::PgPoolOptions, types::time::OffsetDateTime, Pool, Postgres};

use axum::{
    routing::get,
    Router,
};

use polars::prelude::*;

// From: https://stackoverflow.com/questions/73167416/creating-polars-dataframe-from-vecstruct
macro_rules! struct_to_dataframe {
    ($input:expr, [$($field:ident),+]) => {
        {
            let len = $input.len().to_owned();

            // Extract the field values into separate vectors
            $(let mut $field = Vec::with_capacity(len);)*

            for e in $input.into_iter() {
                $($field.push(e.$field);)*
            }

            df! {
                $(stringify!($field) => $field,)*
            }
        }
    };
}

#[derive(Debug)]
struct UserRecord {
    id : i64,
    firstname : Option<String>,
    middlename : Option<String>,
    lastname : Option<String>,
    email : String,
    lastupdate : OffsetDateTime
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub async fn from_postgresql_server(pool : &Pool<Postgres>) -> Result<DataFrame> {

    let out = sqlx::query_as!(UserRecord, "SELECT * FROM UserData")
        .fetch_all(pool)
        .await
        .map_err(|err| err.into() )
        .and_then(|v : Vec<UserRecord>| -> Result<_> {
            struct_to_dataframe!(v, [id, firstname, middlename, lastname, email])
            .map_err(|err| err.into())
            });

    info!("Retrieved data from database");

    out
}