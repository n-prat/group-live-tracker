use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use futures::TryFutureExt;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Row, SqlitePool};

use crate::user::User;

/// Prepare a DB connection pool AND run migrations(eg CREATE TABLE etc)
/// see `https://docs.rs/sqlx/latest/sqlx/macro.migrate.html#`
///
/// params:
/// - `db_url`: &str eg "sqlite://file:db.sqlite?mode=rwc"
pub(crate) async fn setup_db(
    db_url: &str,
    root_user: Option<String>,
    root_password: Option<String>,
) -> Result<SqlitePool, std::io::Error> {
    // Create a connection pool
    //  for MySQL/MariaDB, use MySqlPoolOptions::new()
    //  for SQLite, use SqlitePoolOptions::new()
    //  etc.
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        // mode=rwc means "create if not exists"
        .connect(db_url)
        .map_err(|err| {
            tracing::info!("sqlite connection error: {err:?}",);
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("sqlite connection error: {err:?}",),
            )
        })
        .await?;

    // https://docs.rs/sqlx/latest/sqlx/macro.migrate.html#
    sqlx::migrate!("../server/migrations")
        .run(&pool)
        .await
        .map_err(|err| {
            tracing::info!("sqlx::migrate error: {err:?}",);
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("sqlx::migrate error: {err:?}",),
            )
        })?;

    // TODO if both root_user and root_password are given: INSERT or UPDATE the user and their password
    // if let (Some(root_user), Some(root_password)) = (root_user, root_password) {
    //     match get_user_from_db(&pool, &root_user).await? {
    //         Some(user) => {
    //             // Handle the case when the user exists in the database
    //             // nothing to do
    //         }
    //         None => {
    //             insert_user(&pool, &root_user, &root_password).await?;
    //         }
    //     }

    //     update_user_to_superuser(&pool, &root_user).await?;
    //     update_user_password(&pool, &root_user, &root_password).await?;
    // } else {
    //     tracing::info!("missing root_user and/or root_password; skiping superuser creation",);
    // }

    Ok(pool)
}

/// INSERT a new user, with a random "salt"
/// `https://gemini.google.com`
///
/// returns: the `password_hash`; mostly for tests
// TODO add corresponding route
#[allow(dead_code)]
pub(crate) async fn insert_user(
    pool: &SqlitePool,
    username: &str,
    password: &str,
) -> Result<String, std::io::Error> {
    let password_hash = generate_new_password_hash(password)?;

    let query = r"INSERT INTO user (username, password_hash) VALUES (?, ?)";
    sqlx::query(query)
        .bind(username)
        .bind(&password_hash)
        .execute(pool)
        .map_err(|err| {
            tracing::error!("sqlite query error: {err:?}",);
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("sqlite query error: {err:?}",),
            )
        })
        .await?;

    Ok(password_hash)
}

fn generate_new_password_hash(password: &str) -> Result<String, std::io::Error> {
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|err| {
            tracing::error!("insert_user: password hash error: {err:?}",);
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("insert_user: password hash error: {err:?}",),
            )
        })?
        .to_string();

    Ok(password_hash)
}

/// verify a User's password
/// NOTE: this DOES NOT do a DB query
/// to get a user: use `get_user_from_db`
pub(crate) async fn user_check_password(
    user: &User,
    password_to_check: &str,
) -> Result<(), std::io::Error> {
    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Verify password against PHC string.
    //
    // NOTE: hash params from `parsed_hash` are used instead of what is configured in the
    // `Argon2` instance.
    let parsed_hash_from_db = PasswordHash::new(&user.password_hash).map_err(|err| {
        tracing::error!("select_user_and_check_password: PasswordHash  error: {err:?}",);
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("select_user_and_check_password: PasswordHash error: {err:?}",),
        )
    })?;

    argon2
        .verify_password(password_to_check.as_bytes(), &parsed_hash_from_db)
        .map_err(|err| {
            tracing::error!("select_user_and_check_password: password verify error: {err:?}",);
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("select_user_and_check_password: password verify error: {err:?}",),
            )
        })?;

    Ok(())
}

/// simply check if a user is in the db or not
/// reminder: we want "anynomous" users to be able to access the app
pub(crate) async fn get_user_from_db(
    pool: &SqlitePool,
    username: &str,
) -> Result<Option<User>, std::io::Error> {
    let query = r"
        SELECT username, password_hash, is_super_user FROM user
        WHERE username = $1
    ";
    let row = match sqlx::query(query).bind(username).fetch_one(pool).await {
        Ok(row) => row,
        Err(err) => {
            if let sqlx::Error::RowNotFound = err {
                // no user with that username
                return Ok(None);
            }

            tracing::error!("sqlite query error: {err:?}",);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("sqlite query error: {err:?}",),
            ));
        }
    };

    let user = User {
        username: row.get("username"),
        password_hash: row.get("password_hash"),
        is_super_user: row.get("is_super_user"),
    };

    Ok(Some(user))
}

/// simply check if a user is in the db or not
/// reminder: we want "anynomous" users to be able to access the app
pub(crate) async fn list_users_from_db(pool: &SqlitePool) -> Result<Vec<User>, std::io::Error> {
    let query = r"
        SELECT username, password_hash, is_super_user FROM user
    ";
    let rows = match sqlx::query(query).fetch_all(pool).await {
        Ok(row) => row,
        Err(err) => {
            tracing::error!("sqlite query error: {err:?}",);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("sqlite query error: {err:?}",),
            ));
        }
    };

    let mut all_users = vec![];
    for row in rows {
        let user = User {
            username: row.get("username"),
            password_hash: row.get("password_hash"),
            is_super_user: row.get("is_super_user"),
        };

        all_users.push(user);
    }

    Ok(all_users)
}

/// UPDATE a given user to be a super user
pub(crate) async fn update_user_to_superuser(
    pool: &SqlitePool,
    username: &str,
) -> Result<(), std::io::Error> {
    let query = r"
        UPDATE user
        SET is_super_user = 1
        WHERE username = $1
    ";
    sqlx::query(query)
        .bind(username)
        .execute(pool)
        .map_err(|err| {
            tracing::error!("sqlite query error: {err:?}",);
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("sqlite query error: {err:?}",),
            )
        })
        .await?;

    Ok(())
}

/// UPDATE a given user password
pub(crate) async fn update_user_password(
    pool: &SqlitePool,
    username: &str,
    password: &str,
) -> Result<(), std::io::Error> {
    let password_hash = generate_new_password_hash(password)?;

    let query = r"
        UPDATE user
        SET password_hash = $2
        WHERE username = $1
    ";
    sqlx::query(query)
        .bind(username)
        .bind(password_hash)
        .execute(pool)
        .map_err(|err| {
            tracing::error!("sqlite query error: {err:?}",);
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("sqlite query error: {err:?}",),
            )
        })
        .await?;

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

        let username = "aaa";
        let password_hash = insert_user(&db_pool, username, "bbb").await.unwrap();
        let user = User {
            username: username.to_string(),
            password_hash,
            is_super_user: false,
        };

        assert!(user_check_password(&user, "bbb").await.is_ok());
    }

    #[sqlx::test]
    async fn test_user_wrong_password_should_fail() {
        let db_pool = setup_db("sqlite::memory:").await.unwrap();

        let username = "aaa";
        let password_hash = insert_user(&db_pool, username, "bbb").await.unwrap();
        let user = User {
            username: username.to_string(),
            password_hash,
            is_super_user: false,
        };

        let res = user_check_password(&user, "BAD PASSWORD").await;
        assert!(res.is_err());
    }

    #[sqlx::test]
    async fn test_is_user_in_db_existing_user_return_ok() {
        let db_pool = setup_db("sqlite::memory:").await.unwrap();

        let password_hash = insert_user(&db_pool, "aaa", "bbb").await.unwrap();

        let res = get_user_from_db(&db_pool, "aaa").await;
        assert!(res.is_ok());
        let maybe_user = res.unwrap();
        assert!(maybe_user.is_some());
        assert_eq!(
            maybe_user.unwrap(),
            User {
                username: "aaa".to_string(),
                password_hash,
                is_super_user: false,
            }
        );
    }

    #[sqlx::test]
    async fn test_is_user_in_db_non_existent_user_does_not_err() {
        let db_pool = setup_db("sqlite::memory:").await.unwrap();

        let res = get_user_from_db(&db_pool, "aaa").await;
        assert!(res.is_ok());
        let maybe_user = res.unwrap();
        assert!(maybe_user.is_none());
    }

    #[sqlx::test]
    async fn test_update_user_to_superuser_ok() {
        let db_pool = setup_db("sqlite::memory:").await.unwrap();

        let username = "aaa";
        let password_hash = insert_user(&db_pool, username, "bbb").await.unwrap();

        let res = get_user_from_db(&db_pool, username).await;
        assert_eq!(
            res.unwrap().unwrap(),
            User {
                username: username.to_string(),
                password_hash: password_hash.clone(),
                is_super_user: false,
            }
        );

        assert!(update_user_to_superuser(&db_pool, username).await.is_ok());

        let res = get_user_from_db(&db_pool, username).await;
        assert_eq!(
            res.unwrap().unwrap(),
            User {
                username: username.to_string(),
                password_hash,
                is_super_user: true,
            }
        );
    }

    #[sqlx::test]
    async fn test_list_users_from_db_ok() {
        let db_pool = setup_db("sqlite::memory:").await.unwrap();

        let username1 = "111";
        let password_hash1 = insert_user(&db_pool, username1, "bbb").await.unwrap();
        let username2 = "222";
        let password_hash2 = insert_user(&db_pool, username2, "bbb").await.unwrap();
        let username3 = "333";
        let password_hash3 = insert_user(&db_pool, username3, "bbb").await.unwrap();

        let res = list_users_from_db(&db_pool).await.unwrap();
        assert_eq!(
            res,
            vec![
                User {
                    username: username1.to_string(),
                    password_hash: password_hash1.clone(),
                    is_super_user: false,
                },
                User {
                    username: username2.to_string(),
                    password_hash: password_hash2.clone(),
                    is_super_user: false,
                },
                User {
                    username: username3.to_string(),
                    password_hash: password_hash3.clone(),
                    is_super_user: false,
                }
            ]
        );
    }
}
