use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use futures::TryFutureExt;
use sqlx::Row;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

/// Prepare a DB connection pool AND run migrations(eg CREATE TABLE etc)
/// see https://docs.rs/sqlx/latest/sqlx/macro.migrate.html#
///
/// params:
/// - db_url: &str eg "sqlite://file:db.sqlite?mode=rwc"
pub(crate) async fn setup_db(db_url: &str) -> Result<Pool<Sqlite>, std::io::Error> {
    // Create a connection pool
    //  for MySQL/MariaDB, use MySqlPoolOptions::new()
    //  for SQLite, use SqlitePoolOptions::new()
    //  etc.
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        // mode=rwc means "create if not exists"
        .connect(db_url)
        .map_err(|err| {
            tracing::info!("sqlite connection error: {:?}", err,);
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("sqlite connection error: {:?}", err,),
            )
        })
        .await?;

    // https://docs.rs/sqlx/latest/sqlx/macro.migrate.html#
    sqlx::migrate!("../server/migrations")
        .run(&pool)
        .await
        .map_err(|err| {
            tracing::info!("sqlx::migrate error: {:?}", err,);
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("sqlx::migrate error: {:?}", err,),
            )
        })?;

    Ok(pool)
}

/// INSERT a new user, with a random "salt"
/// https://gemini.google.com
async fn insert_user(
    pool: &Pool<Sqlite>,
    username: &str,
    password: &str,
) -> Result<(), std::io::Error> {
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|err| {
            tracing::error!("insert_user: password hash error: {:?}", err,);
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("insert_user: password hash error: {:?}", err,),
            )
        })?
        .to_string();

    let query = r#"INSERT INTO user (username, password_hash) VALUES (?, ?)"#;
    sqlx::query(query)
        .bind(username)
        .bind(&password_hash)
        .execute(pool)
        .map_err(|err| {
            tracing::error!("sqlite query error: {:?}", err,);
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("sqlite query error: {:?}", err,),
            )
        })
        .await?;

    Ok(())
}

/// SELECT a user and verify their password
async fn select_user_and_check_password(
    pool: &Pool<Sqlite>,
    username: &str,
    password_to_check: &str,
) -> Result<(), std::io::Error> {
    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    let query = r#"
        SELECT username, password_hash FROM user
        WHERE username = $1
    "#;
    let row = sqlx::query(query)
        .bind(username)
        .fetch_one(pool)
        .map_err(|err| {
            tracing::error!("sqlite query error: {:?}", err,);
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("sqlite query error: {:?}", err,),
            )
        })
        .await?;

    // Verify password against PHC string.
    //
    // NOTE: hash params from `parsed_hash` are used instead of what is configured in the
    // `Argon2` instance.
    let password_hash_from_db: String = row.get("password_hash");
    let parsed_hash_from_db = PasswordHash::new(&password_hash_from_db).map_err(|err| {
        tracing::error!(
            "select_user_and_check_password: PasswordHash  error: {:?}",
            err,
        );
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!(
                "select_user_and_check_password: PasswordHash error: {:?}",
                err,
            ),
        )
    })?;

    argon2
        .verify_password(password_to_check.as_bytes(), &parsed_hash_from_db)
        .map_err(|err| {
            tracing::error!(
                "select_user_and_check_password: password verify error: {:?}",
                err,
            );
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "select_user_and_check_password: password verify error: {:?}",
                    err,
                ),
            )
        })?;

    Ok(())
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    #[sqlx::test]
    async fn test_can_not_have_two_users_with_same_username() {
        let db_pool = setup_db("sqlite::memory:").await.unwrap();

        insert_user(&db_pool, "aaa", "bbb").await.unwrap();

        assert!(insert_user(&db_pool, "aaa", "ccc").await.is_err());
    }

    #[sqlx::test]
    async fn test_can_have_two_users_with_different_usernames() {
        let db_pool = setup_db("sqlite::memory:").await.unwrap();

        insert_user(&db_pool, "aaa", "bbb").await.unwrap();

        assert!(insert_user(&db_pool, "bbb", "ccc").await.is_ok());
    }

    #[sqlx::test]
    async fn test_user_good_password_ok() {
        let db_pool = setup_db("sqlite::memory:").await.unwrap();

        insert_user(&db_pool, "aaa", "bbb").await.unwrap();

        assert!(select_user_and_check_password(&db_pool, "aaa", "bbb")
            .await
            .is_ok());
    }

    #[sqlx::test]
    async fn test_user_wrong_password_should_fail() {
        let db_pool = setup_db("sqlite::memory:").await.unwrap();

        insert_user(&db_pool, "aaa", "bbb").await.unwrap();

        let res = select_user_and_check_password(&db_pool, "aaa", "BAD PASSWORD").await;
        assert!(res.is_err());
    }
}
