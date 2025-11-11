use async_trait::async_trait;
use diesel::prelude::*;
use diesel::result::{self, DatabaseErrorKind, Error};
use diesel_async::scoped_futures::ScopedFutureExt;
use diesel_async::{AsyncConnection, RunQueryDsl};
use tokio::sync::MutexGuard;

use crate::data::{
    database::{connect_from_pool, lock_db},
    models::publishers::{NewPublisher, Publishers, UpdatePublisher},
    repos::traits::repository::Repository,
};

// TODO: Test this
pub struct PublisherRepo;

impl PublisherRepo {
    pub async fn new() -> Self {
        PublisherRepo
    }

    pub async fn search_by_name(
        &self,
        name_query: &str,
    ) -> Result<Option<Vec<Publishers>>, result::Error> {
        use crate::data::models::schema::publishers::dsl::*;

        let mut conn = connect_from_pool().await.map_err(|e| {
            Error::DatabaseError(
                DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string()),
            )
        })?;

        return match publishers
            .filter(name.like(format!("%{}%", name_query)))
            .load::<Publishers>(&mut conn)
            .await
        {
            Ok(value) => Ok(Some(value)),
            Err(Error::NotFound) => Ok(None),
            Err(e) => Err(e),
        };
    }
}

#[async_trait]
impl Repository for PublisherRepo {
    type Item = Publishers;
    type NewItem<'a> = NewPublisher<'a>;
    type Form<'a> = UpdatePublisher<'a>;
    type Id = i32;

    async fn get_all(&self) -> Result<Option<Vec<Self::Item>>, result::Error> {
        use crate::data::models::schema::publishers::dsl::*;

        let mut conn = connect_from_pool().await.map_err(|e| {
            result::Error::DatabaseError(
                DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string()),
            )
        })?;

        match publishers.load::<Self::Item>(&mut conn).await {
            Ok(value) => Ok(Some(value)),
            Err(result::Error::NotFound) => Ok(None),
            Err(e) => Err(e),
        }
    }

    async fn get_by_id(&self, id: Self::Id) -> Result<Option<Self::Item>, result::Error> {
        use crate::data::models::schema::{publishers as publisher, publishers::dsl::*};

        let mut conn = connect_from_pool().await.map_err(|e| {
            Error::DatabaseError(
                DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string()),
            )
        })?;

        match publishers
            .filter(publisher::publisher_id.eq(id))
            .first::<Publishers>(&mut conn)
            .await
        {
            Ok(value) => Ok(Some(value)),
            Err(Error::NotFound) => Ok(None),
            Err(e) => Err(e),
        }
    }

    async fn add<'a>(&self, new_item: Self::NewItem<'a>) -> Result<(), result::Error> {
        use crate::data::models::schema::publishers::dsl::*;

        let mut conn = connect_from_pool().await.map_err(|e| {
            Error::DatabaseError(
                DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string()),
            )
        })?;

        let db_lock = lock_db();
        let _guard: MutexGuard<()> = db_lock.lock().await;

        match conn
            .transaction(|connection| {
                async move {
                    diesel::insert_into(publishers)
                        .values(new_item)
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

    async fn update<'a>(
        &self,
        id: Self::Id,
        updated_item: Self::Form<'a>,
    ) -> Result<(), result::Error> {
        use crate::data::models::schema::publishers::dsl::*;

        let mut conn = connect_from_pool().await.map_err(|e| {
            Error::DatabaseError(
                DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string()),
            )
        })?;

        let db_lock = lock_db();
        let _guard: MutexGuard<()> = db_lock.lock().await;

        match conn
            .transaction(|connection| {
                async move {
                    diesel::update(publishers.filter(publisher_id.eq(id)))
                        .set(updated_item)
                        .execute(connection)
                        .await?;

                    Ok(())
                }
                .scope_boxed()
            })
            .await
        {
            Ok(publisher) => Ok(publisher),
            Err(e) => Err(e),
        }
    }

    async fn delete(&self, id: Self::Id) -> Result<(), result::Error> {
        use crate::data::models::schema::publishers::dsl::*;

        let mut conn = connect_from_pool().await.map_err(|e| {
            Error::DatabaseError(
                DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string()),
            )
        })?;

        let db_lock = lock_db();
        let _guard: MutexGuard<()> = db_lock.lock().await;

        match conn
            .transaction(|connection| {
                async move {
                    diesel::delete(publishers.filter(publisher_id.eq(id)))
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
