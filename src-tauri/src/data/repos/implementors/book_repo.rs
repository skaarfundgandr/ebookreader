use diesel::prelude::*;
use diesel::query_dsl::methods::FilterDsl;
use diesel::result::{self, DatabaseErrorKind, Error};
use diesel_async::scoped_futures::ScopedFutureExt;
use diesel_async::{AsyncConnection, RunQueryDsl};
use async_trait::async_trait;
use tokio::sync::MutexGuard;

use crate::data::{
    models::books::{Books, NewBook, UpdateBook},
    repos::traits::repository::Repository,
    database::{connect_from_pool, lock_db},
};

// TODO: Test this
pub struct BookRepo;

impl BookRepo {
    pub fn new() -> Self {
        BookRepo
    }

    pub async fn search_by_title(&self, title_query: &str) -> Result<Option<Vec<Books>>, result::Error> {
        use crate::data::models::schema::books::dsl::*;

        let mut conn = connect_from_pool().await.map_err(|e| {
            Error::DatabaseError(
                DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string()),
            )
        })?;

        return match books
            .filter(title.like(format!("%{}%", title_query)))
            .load::<Books>(&mut conn)
            .await
        {
            Ok(value) => Ok(Some(value)),
            Err(Error::NotFound) => Ok(None),
            Err(e) => Err(e),
        };
    }

    pub async fn search_by_publisher(&self, pub_id: i32) -> Result<Option<Vec<Books>>, result::Error> {
        use crate::data::models::schema::books::dsl::*;

        let mut conn = connect_from_pool().await.map_err(|e| {
            Error::DatabaseError(
                DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string()),
            )
        })?;

        return match books
            .filter(publisher_id.eq(pub_id))
            .load::<Books>(&mut conn)
            .await
        {
            Ok(value) => Ok(Some(value)),
            Err(Error::NotFound) => Ok(None),
            Err(e) => Err(e),
        };
    }
}

#[async_trait]
impl Repository for BookRepo {
    type Item = Books;
    type NewItem = NewBook<'static>;
    type Form = UpdateBook<'static>;
    type Id = i32;

    async fn get_all(&self) -> Result<Option<Vec<Self::Item>>, result::Error> {
        use crate::data::models::schema::books::dsl::*;

        let mut conn = connect_from_pool().await.map_err(|e| {
            result::Error::DatabaseError(
                DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string()),
            )
        })?;

        match books.load::<Self::Item>(&mut conn).await {
            Ok(value) => Ok(Some(value)),
            Err(result::Error::NotFound) => Ok(None),
            Err(e) => Err(e),
        }
    }

    async fn get_by_id(&self, id: Self::Id) -> Result<Option<Self::Item>, result::Error> {
        use crate::data::models::schema::{books as book, books::dsl::*};

        let mut conn = connect_from_pool().await.map_err(|e| {
            Error::DatabaseError(
                DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string()),
            )
        })?;

        match books
            .filter(book::book_id.eq(id))
            .first::<Books>(&mut conn)
            .await
        {
            Ok(value) => Ok(Some(value)),
            Err(Error::NotFound) => Ok(None),
            Err(e) => Err(e),
        }
    }

    async fn add(&self, new_item: Self::NewItem) -> Result<Self::Item, result::Error> {
        use crate::data::models::schema::books::dsl::*;

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
                diesel::insert_into(books)
                    .values(&new_item)
                    .execute(connection)
                    .await?;

                // Fetch the inserted book (best-effort: get most recent)
                let inserted = books
                    .order(book_id.desc())
                    .first::<Books>(connection)
                    .await?;
                
                Ok(inserted)
            }
            .scope_boxed()
        })
        .await
    }

    async fn update(&self, id: Self::Id, updated_item: Self::Form) -> Result<(), result::Error> {
        use crate::data::models::schema::books::dsl::*;

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
                diesel::update(books.filter(book_id.eq(id)))
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
        use crate::data::models::schema::books::dsl::*;

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
                diesel::delete(books.filter(book_id.eq(id)))
                    .execute(connection)
                    .await?;
                Ok(())
            }
            .scope_boxed()
        })
        .await
    }
}
