use async_trait::async_trait;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error};
use diesel_async::scoped_futures::ScopedFutureExt;
use diesel_async::{AsyncConnection, RunQueryDsl};
use tokio::sync::MutexGuard;

use crate::data::{
    database::{connect_from_pool, lock_db},
    models::annotations::{Annotations, NewAnnotation, UpdateAnnotation},
    repos::traits::repository::Repository,
};

pub struct AnnotationRepo;

impl AnnotationRepo {
    pub async fn new() -> Self {
        AnnotationRepo
    }

    pub async fn get_by_user_and_book(
        &self,
        user_id: i32,
        book_id: i32,
    ) -> Result<Option<Vec<Annotations>>, Error> {
        use crate::data::models::schema::annotations::dsl::*;

        let mut conn = connect_from_pool().await.map_err(|e| {
            Error::DatabaseError(
                DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string()),
            )
        })?;

        match annotations
            .filter(crate::data::models::schema::annotations::user_id.eq(user_id))
            .filter(crate::data::models::schema::annotations::book_id.eq(book_id))
            .load::<Annotations>(&mut conn)
            .await
        {
            Ok(value) if value.is_empty() => Ok(None),
            Ok(value) => Ok(Some(value)),
            Err(Error::NotFound) => Ok(None),
            Err(e) => Err(e),
        }
    }
}

#[async_trait]
impl Repository for AnnotationRepo {
    type Item = Annotations;
    type NewItem<'a> = NewAnnotation<'a>;
    type Form<'a> = UpdateAnnotation<'a>;
    type Id = i32;

    async fn get_all(&self) -> Result<Option<Vec<Self::Item>>, Error> {
        use crate::data::models::schema::annotations::dsl::*;

        let mut conn = connect_from_pool().await.map_err(|e| {
            Error::DatabaseError(
                DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string()),
            )
        })?;

        match annotations.load::<Self::Item>(&mut conn).await {
            Ok(value) if value.is_empty() => Ok(None),
            Ok(value) => Ok(Some(value)),
            Err(Error::NotFound) => Ok(None),
            Err(e) => Err(e),
        }
    }

    async fn get_by_id(&self, id: Self::Id) -> Result<Option<Self::Item>, Error> {
        use crate::data::models::schema::annotations::dsl::*;

        let mut conn = connect_from_pool().await.map_err(|e| {
            Error::DatabaseError(
                DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string()),
            )
        })?;

        match annotations
            .filter(annotation_id.eq(id))
            .first::<Annotations>(&mut conn)
            .await
        {
            Ok(value) => Ok(Some(value)),
            Err(Error::NotFound) => Ok(None),
            Err(e) => Err(e),
        }
    }

    async fn add<'a>(&self, new_item: Self::NewItem<'a>) -> Result<(), Error> {
        use crate::data::models::schema::annotations::dsl::*;

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
                diesel::insert_into(annotations)
                    .values(new_item)
                    .execute(connection)
                    .await?;
                Ok(())
            }
            .scope_boxed()
        })
        .await
    }

    async fn update<'a>(&self, id: Self::Id, updated_item: Self::Form<'a>) -> Result<(), Error> {
        use crate::data::models::schema::annotations::dsl::*;

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
                diesel::update(annotations.filter(annotation_id.eq(id)))
                    .set(updated_item)
                    .execute(connection)
                    .await?;
                Ok(())
            }
            .scope_boxed()
        })
        .await
    }

    async fn delete(&self, id: Self::Id) -> Result<(), Error> {
        use crate::data::models::schema::annotations::dsl::*;

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
                diesel::delete(annotations.filter(annotation_id.eq(id)))
                    .execute(connection)
                    .await?;
                Ok(())
            }
            .scope_boxed()
        })
        .await
    }
}
