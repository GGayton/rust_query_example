# rust-query-example

The aim of this project is to 
1. Spin up a postgresql db of user data
2. Spin up a rust web server
3. On get call to server:
    1. Obtain user data from db
    2. Construct a polars lazyframe
    3. Return the table 

To update sqlx query macros, run:
```
cargo sqlx prepare --database-url "postgres://user:admin@localhost/users"
```


To spin up our postgresql db, run:
```
docker compose up -d --build
```