use diesel::prelude::*;
use diesel::result::*;
use diesel_async::scoped_futures::ScopedFutureExt;
use diesel_async::AsyncConnection;
use diesel_async::RunQueryDsl;
use tokio::sync::MutexGuard;

use crate::controllers::dto::user_dto::NewUserDTO;
use crate::controllers::dto::user_dto::UserDTO;
use crate::data::database::*;
use crate::data::models::users::NewUser;
use crate::data::models::users::Users;
// TODO: Make this implement the repository trait and deprecate after
pub async fn get_all_users() -> Result<Option<Vec<Users>>, Error> {
    use crate::data::models::schema::users::dsl::*;

    let mut conn = connect_from_pool().await.map_err(|e| {
        Error::DatabaseError(
            DatabaseErrorKind::UnableToSendCommand,
            Box::new(e.to_string()),
        )
    })?;

    return match users.load::<Users>(&mut conn).await {
        Ok(value) => Ok(Some(value)),
        Err(Error::NotFound) => Ok(None),
        Err(e) => Err(e),
    };
}

pub async fn get_user_by_id(id: i32) -> Result<Option<UserDTO>, Error> {
    use crate::data::models::schema::{users as user, users::dsl::*};

    let mut conn = connect_from_pool().await.map_err(|e| {
        Error::DatabaseError(
            DatabaseErrorKind::UnableToSendCommand,
            Box::new(e.to_string()),
        )
    })?;

    return match users
        .filter(user::user_id.eq(id))
        .first::<Users>(&mut conn)
        .await
    {
        Ok(value) => Ok(Some(UserDTO::from(value))),
        Err(Error::NotFound) => Ok(None),
        Err(e) => Err(e),
    };
}

pub async fn get_user_by_username(user_name: &str) -> Result<Option<UserDTO>, Error> {
    use crate::data::models::schema::users::dsl::*;

    let mut conn = connect_from_pool().await.map_err(|e| {
        Error::DatabaseError(
            DatabaseErrorKind::UnableToSendCommand,
            Box::new(e.to_string()),
        )
    })?;

    return match users
        .filter(username.eq(user_name))
        .first::<Users>(&mut conn)
        .await
    {
        Ok(value) => Ok(Some(UserDTO::from(value))),
        Err(Error::NotFound) => Ok(None),
        Err(e) => Err(e),
    };
}
pub async fn create_user(new_user_dto: NewUserDTO) -> Result<(), Error> {
    use crate::data::models::schema::users::dsl::*;

    let mut conn = connect_from_pool().await.map_err(|e| {
        Error::DatabaseError(
            DatabaseErrorKind::UnableToSendCommand,
            Box::new(e.to_string()),
        )
    })?;

    let db_lock = lock_db();
    let _guard: MutexGuard<()> = db_lock.lock().await;

    let new_user = NewUser {
        username: &new_user_dto.username,
        email: &new_user_dto.email,
        password_hash: &new_user_dto.password_hash,
        created_at: None,
    };

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
