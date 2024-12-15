use tracing::info;
use sqlx::{types::time::OffsetDateTime, Pool, Postgres};

use polars::prelude::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// From: https://stackoverflow.com/questions/73167416/creating-polars-dataframe-from-vecstruct
/// Modified as to allow mappings for custom objects 
/// Modified to use LazyFrame constructor
macro_rules! struct_to_dataframe {
    ($input:expr, [$($field:ident, $map:expr),+]) => {
        {
            let len = $input.len().to_owned();

            // Extract the field values into separate vectors
            $(let mut $field = Vec::with_capacity(len);)*

            for e in $input.into_iter() {
                $($field.push($map(e.$field));)*
            }

            // I imagine these methods are equivalent, we have already done the work to 
            // create our vectorised data.
            //df! {
                //$(stringify!($field) => $field,)*
            //}

            let df = LazyFrame::default();

            df
                $(.with_column( Series::new(stringify!($field).into(), $field).lit()))+
                
        }
    };
}

/// Record used for translation from postgresql db
#[derive(Debug)]
struct UserRecord {
    id : i64,
    firstname : Option<String>,
    middlename : Option<String>,
    lastname : Option<String>,
    email : String,
    lastupdate : OffsetDateTime
}

/// Constructs a dataframe from a vector of user records
fn from_user_record(vec : Vec<UserRecord>) -> LazyFrame {
    
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
        .with_column(
                    col("lastupdate")
                    .cast(DataType::Datetime(
                        TimeUnit::Milliseconds, 
                        Some("Utc".into()))
                    )
                )
            
        
}

/// Obtains data from the postgresql db, limited to 10 rows only
pub async fn from_postgresql_server(pool : &Pool<Postgres>) -> Result<LazyFrame> {

    let out : Result<LazyFrame> = sqlx::query_as!(UserRecord, "SELECT * FROM UserData LIMIT 10 OFFSET 0")
        .fetch_all(pool)
        .await
        .map_err(|err| err.into() )
        .and_then(|v| Ok(from_user_record(v)));
    
    info!("Retrieved data from database");

    out
}