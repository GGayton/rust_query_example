use tracing::info;
use sqlx::{types::time::OffsetDateTime, Pool, Postgres};

use polars::prelude::*;

/// From: https://stackoverflow.com/questions/73167416/creating-polars-dataframe-from-vecstruct
/// Modified as to allow mappings for custom objects 
macro_rules! struct_to_dataframe {
    ($input:expr, [$($field:ident, $map:expr),+]) => {
        {
            let len = $input.len().to_owned();

            // Extract the field values into separate vectors
            $(let mut $field = Vec::with_capacity(len);)*

            for e in $input.into_iter() {
                $($field.push($map(e.$field));)*
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

fn from_user_record(vec : Vec<UserRecord>) -> Result<DataFrame> {

    
    fn as_is<T>(e : T) -> T { e }

    fn from_offset_date_time(e : OffsetDateTime) -> i64 { e.unix_timestamp()*1000 }

    struct_to_dataframe!(
        vec, 
        [
            id, as_is, 
            firstname, as_is,
            middlename, as_is,
            lastname, as_is,
            email, as_is,
            lastupdate, from_offset_date_time
        ])
        .map(|mut df| {
            df.with_column(
                df
                    .column("lastupdate")
                    .unwrap()
                    .cast(&DataType::Datetime(TimeUnit::Milliseconds, Some("Utc".into())))
                    .unwrap()
                ).expect("Malformed data on last update");
            df
        })
        .map_err(|err| err.into() )
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub async fn from_postgresql_server(pool : &Pool<Postgres>) -> Result<DataFrame> {

    let out = sqlx::query_as!(UserRecord, "SELECT * FROM UserData LIMIT 10 OFFSET 0")
        .fetch_all(pool)
        .await
        .map_err(|err| err.into() )
        .and_then(from_user_record)
        .map_err(|err| err.into());
    
    info!("Retrieved data from database");

    out
}