
To update sqlx query macros, run:
```
cargo sqlx prepare --database-url "postgres://user:admin@localhost/users"
```


To spin up our postgresql db, run:
```
docker compose up -d --build
```