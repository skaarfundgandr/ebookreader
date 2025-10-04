use diesel::prelude::*;
use diesel::result::*;
use diesel_async::scoped_futures::ScopedFutureExt;
use diesel_async::AsyncConnection;
use diesel_async::RunQueryDsl;
use tokio::sync::MutexGuard;

use crate::data::database::*;
use crate::data::models::users::NewUser;
use crate::data::models::users::Users;
//TODO: Acquire mutex lock when adding entry to the database
pub async fn get_all_users() -> Result<Option<Vec<Users>>, Error> {
    use crate::data::models::schema::users::dsl::*;

    let mut conn = connect_from_pool().await.map_err(|e| {
        Error::DatabaseError(
            DatabaseErrorKind::UnableToSendCommand,
            Box::new(e.to_string()),
        )
    })?;

    return match users.select(Users::as_select()).load(&mut conn).await {
        Ok(value) => Ok(Some(value)),
        Err(Error::NotFound) => Ok(None),
        Err(e) => Err(e),
    };
}

pub async fn get_user_by_id(id: i32) -> Result<Option<Users>, Error> {
    use crate::data::models::schema::{users as user, users::dsl::*};

    let mut conn = connect_from_pool().await.map_err(|e| {
        Error::DatabaseError(
            DatabaseErrorKind::UnableToSendCommand,
            Box::new(e.to_string()),
        )
    })?;

    return match users
        .filter(user::user_id.eq(id))
        .select(Users::as_select())
        .first(&mut conn)
        .await
    {
        Ok(value) => Ok(Some(value)),
        Err(Error::NotFound) => Ok(None),
        Err(e) => Err(e),
    };
}

pub async fn get_user_by_username(user_name: &str) -> Result<Option<Users>, Error> {
    use crate::data::models::schema::users::dsl::*;

    let mut conn = connect_from_pool().await.map_err(|e| {
        Error::DatabaseError(
            DatabaseErrorKind::UnableToSendCommand,
            Box::new(e.to_string()),
        )
    })?;

    return match users
        .filter(username.eq(user_name))
        .select(Users::as_select())
        .first(&mut conn)
        .await
    {
        Ok(value) => Ok(Some(value)),
        Err(Error::NotFound) => Ok(None),
        Err(e) => Err(e),
    };
}
// TODO: Acquire mutex lock when adding entry to the database
pub async fn create_user(new_user: NewUser<'_>) -> Result<(), Error> {
    use crate::data::models::schema::users::dsl::*;

    let mut conn = connect_from_pool().await.map_err(|e| {
        Error::DatabaseError(
            DatabaseErrorKind::UnableToSendCommand,
            Box::new(e.to_string()),
        )
    })?;

    let db_lock = lock_db();
    let _guard: MutexGuard<()> = db_lock.lock().await;

    let result = match conn
        .transaction(|connection| {
            async move {
                diesel::insert_into(users)
                    .values(new_user)
                    .execute(connection)
                    .await?;
                                
                Ok(())
            }
            .scope_boxed()
        })
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    };

    return result;
}
