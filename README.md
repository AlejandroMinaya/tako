# tako

## Set up
### Database
1. Get the connection URL to your PostgreSQL database. It should look something like this:
```
postgresql://<username>:<password>@<hostname>:<port>/<dbname>
```
2. Install `sqlx-cli` with `cargo`
```
cargo install sqlx-cli
```
3. Run the database migrations using the connection URL from step `1`.
```
sqlx database setup --database-url <connection_url>
```
