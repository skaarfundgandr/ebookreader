use diesel::prelude::*;
use diesel::result::{self, DatabaseErrorKind, Error};
use diesel_async::scoped_futures::ScopedFutureExt;
use diesel_async::{AsyncConnection, RunQueryDsl};
use async_trait::async_trait;
use tokio::sync::MutexGuard;

use crate::data::{
    models::users::{Users, NewUser, UpdateUser},
    repos::traits::repository::Repository,
    database::{connect_from_pool, lock_db},
};

// TODO: Test this
pub struct UserRepo;

impl UserRepo {
    pub fn new() -> Self {
        UserRepo
    }

    pub async fn search_by_username(&self, username_query: &str) -> Result<Option<Users>, result::Error> {
        use crate::data::models::schema::users::dsl::*;

        let mut conn = connect_from_pool().await.map_err(|e| {
            Error::DatabaseError(
                DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string()),
            )
        })?;

        return match users
            .filter(username.eq(username_query))
            .first::<Users>(&mut conn)
            .await
        {
            Ok(value) => Ok(Some(value)),
            Err(Error::NotFound) => Ok(None),
            Err(e) => Err(e),
        };
    }

    pub async fn search_by_email(&self, email_query: &str) -> Result<Option<Users>, result::Error> {
        use crate::data::models::schema::users::dsl::*;

        let mut conn = connect_from_pool().await.map_err(|e| {
            Error::DatabaseError(
                DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string()),
            )
        })?;

        return match users
            .filter(email.eq(email_query))
            .first::<Users>(&mut conn)
            .await
        {
            Ok(value) => Ok(Some(value)),
            Err(Error::NotFound) => Ok(None),
            Err(e) => Err(e),
        };
    }
}

#[async_trait]
impl Repository for UserRepo {
    type Item = Users;
    type NewItem = NewUser<'static>;
    type Form = UpdateUser<'static>;
    type Id = i32;

    async fn get_all(&self) -> Result<Option<Vec<Self::Item>>, result::Error> {
        use crate::data::models::schema::users::dsl::*;

        let mut conn = connect_from_pool().await.map_err(|e| {
            result::Error::DatabaseError(
                DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string()),
            )
        })?;

        return match users.load::<Self::Item>(&mut conn).await {
            Ok(value) => Ok(Some(value)),
            Err(result::Error::NotFound) => Ok(None),
            Err(e) => Err(e),
        };
    }

    async fn get_by_id(&self, id: Self::Id) -> Result<Option<Self::Item>, result::Error> {
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
            Ok(value) => Ok(Some(value)),
            Err(Error::NotFound) => Ok(None),
            Err(e) => Err(e),
        };
    }

    async fn add(&self, new_item: Self::NewItem) -> Result<Self::Item, result::Error> {
        use crate::data::models::schema::users::dsl::*;

        let mut conn = connect_from_pool().await.map_err(|e| {
            Error::DatabaseError(
                DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string()),
            )
        })?;

        let db_lock = lock_db();
        let _guard: MutexGuard<()> = db_lock.lock().await;

        conn.transaction(|connection| {
            async move {
                diesel::insert_into(users)
                    .values(&new_item)
                    .execute(connection)
                    .await?;

                // Fetch the inserted user (best-effort: get most recent)
                let inserted = users
                    .order(user_id.desc())
                    .first::<Users>(connection)
                    .await?;
                
                Ok(inserted)
            }
            .scope_boxed()
        })
        .await
    }

    async fn update(&self, id: Self::Id, updated_item: Self::Form) -> Result<(), result::Error> {
        use crate::data::models::schema::users::dsl::*;

        let mut conn = connect_from_pool().await.map_err(|e| {
            Error::DatabaseError(
                DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string()),
            )
        })?;

        let db_lock = lock_db();
        let _guard: MutexGuard<()> = db_lock.lock().await;

        conn.transaction(|connection| {
            async move {
                diesel::update(users.filter(user_id.eq(id)))
                    .set(&updated_item)
                    .execute(connection)
                    .await?;

                Ok(())
            }
            .scope_boxed()
        })
        .await
    }

    async fn delete(&self, id: Self::Id) -> Result<(), result::Error> {
        use crate::data::models::schema::users::dsl::*;

        let mut conn = connect_from_pool().await.map_err(|e| {
            Error::DatabaseError(
                DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string()),
            )
        })?;

        let db_lock = lock_db();
        let _guard: MutexGuard<()> = db_lock.lock().await;

        conn.transaction(|connection| {
            async move {
                diesel::delete(users.filter(user_id.eq(id)))
                    .execute(connection)
                    .await?;
                Ok(())
            }
            .scope_boxed()
        })
        .await
    }
}
 