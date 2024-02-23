use futures::TryFutureExt;
use sqlx::{
    sqlite::{SqlitePoolOptions, SqliteQueryResult},
    Pool, Sqlite,
};

pub(crate) async fn setup_db() -> Result<Pool<Sqlite>, std::io::Error> {
    // Create a connection pool
    //  for MySQL/MariaDB, use MySqlPoolOptions::new()
    //  for SQLite, use SqlitePoolOptions::new()
    //  etc.
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        // mode=rwc means "create if not exists"
        .connect("sqlite://file:db.sqlite?mode=rwc")
        .map_err(|err| {
            tracing::info!("sqlite connection error: {:?}", err,);
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("sqlite connection error: {:?}", err,),
            )
        })
        .await?;

    create_table_user(&pool).await?;

    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL/MariaDB)
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .map_err(|err| {
            tracing::error!("sqlite query error: {:?}", err,);
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("sqlite query error: {:?}", err,),
            )
        })
        .await?;
    tracing::info!("sqlite result: {:?}", row,);

    Ok(pool)
}

async fn create_table_user(pool: &Pool<Sqlite>) -> Result<SqliteQueryResult, std::io::Error> {
    let query = r#"
        CREATE TABLE IF NOT EXISTS user (
            id INTEGER NOT NULL PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            is_super_user BOOLEAN NOT NULL
        );
    "#;
    sqlx::query(query)
        .execute(pool)
        .map_err(|err| {
            tracing::error!("sqlite query error: {:?}", err,);
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("sqlite query error: {:?}", err,),
            )
        })
        .await
}
