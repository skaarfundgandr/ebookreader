use async_trait::async_trait;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error};
use diesel_async::{scoped_futures::ScopedFutureExt, AsyncConnection, RunQueryDsl};

use crate::data::{
    database::{connect_from_pool, lock_db},
    models::libraries::{Library, NewLibrary, UpdateLibrary},
    repos::traits::repository::Repository,
};

pub struct LibraryRepo;

impl LibraryRepo {
    pub async fn new() -> Self {
        LibraryRepo
    }

    pub async fn search_by_name(&self, name_query: &str) -> Result<Option<Vec<Library>>, Error> {
        use crate::data::models::schema::libraries::dsl::*;

        let mut conn = connect_from_pool().await.map_err(|e| {
            Error::DatabaseError(
                DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string()),
            )
        })?;

        return match libraries
            .filter(name.like(format!("%{}%", name_query)))
            .load::<Library>(&mut conn)
            .await
        {
            Ok(value) if value.is_empty() => Ok(None),
            Ok(value) => Ok(Some(value)),
            Err(Error::NotFound) => Ok(None),
            Err(e) => Err(e),
        };
    }
}

#[async_trait]
impl Repository for LibraryRepo {
    type Item = Library;
    type NewItem<'a> = NewLibrary<'a>;
    type Form<'a> = UpdateLibrary<'a>;
    type Id = i32;

    async fn get_all(&self) -> Result<Option<Vec<Self::Item>>, Error> {
        use crate::data::models::schema::libraries::dsl::*;

        let mut conn = connect_from_pool().await.map_err(|e| {
            Error::DatabaseError(
                DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string()),
            )
        })?;

        match libraries.load::<Self::Item>(&mut conn).await {
            Ok(value) if value.is_empty() => Ok(None),
            Ok(value) => Ok(Some(value)),
            Err(Error::NotFound) => Ok(None),
            Err(e) => Err(e),
        }
    }

    async fn get_by_id(&self, id: Self::Id) -> Result<Option<Self::Item>, Error> {
        use crate::data::models::schema::{libraries as lib, libraries::dsl::*};

        let mut conn = connect_from_pool().await.map_err(|e| {
            Error::DatabaseError(
                DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string()),
            )
        })?;

        match libraries
            .filter(lib::library_id.eq(id))
            .first::<Library>(&mut conn)
            .await
        {
            Ok(value) => Ok(Some(value)),
            Err(Error::NotFound) => Ok(None),
            Err(e) => Err(e),
        }
    }

    async fn add<'a>(&self, item: Self::NewItem<'a>) -> Result<(), Error> {
        use crate::data::models::schema::libraries::dsl::*;

        let mut conn = connect_from_pool().await.map_err(|e| {
            Error::DatabaseError(
                DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string()),
            )
        })?;

        let db_lock = lock_db();
        let _guard = db_lock.lock().await;

        match conn
            .transaction(|connection| {
                async move {
                    diesel::insert_into(libraries)
                        .values(&item)
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
        }
    }

    async fn update<'a>(&self, id: Self::Id, item: Self::Form<'a>) -> Result<(), Error> {
        use crate::data::models::schema::libraries::dsl::*;

        let mut conn = connect_from_pool().await.map_err(|e| {
            Error::DatabaseError(
                DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string()),
            )
        })?;

        let db_lock = lock_db();
        let _guard = db_lock.lock().await;

        match conn
            .transaction(|connection| {
                async move {
                    diesel::update(libraries.filter(library_id.eq(id)))
                        .set(&item)
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
        }
    }

    async fn delete(&self, id: Self::Id) -> Result<(), Error> {
        use crate::data::models::schema::libraries::dsl::*;

        let mut conn = connect_from_pool().await.map_err(|e| {
            Error::DatabaseError(
                DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string()),
            )
        })?;

        let db_lock = lock_db();
        let _guard = db_lock.lock().await;

        match conn
            .transaction(|connection| {
                async move {
                    diesel::delete(libraries.filter(library_id.eq(id)))
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
        }
    }
}
