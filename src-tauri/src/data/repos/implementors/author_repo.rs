use diesel::result;
use async_trait::async_trait;

use crate::data::{
    models::authors::{
        Authors,
        NewAuthor,
        AuthorForm
    },
    repos::traits::repository::Repository,
    database::connect_from_pool,
};
// TODO: Test this
pub struct AuthorRepo;

impl AuthorRepo {
    async fn new() -> Self {
        AuthorRepo
    }

    async fn search_by_name(&self, name_query: &str) -> Result<Option<Vec<Authors>>, result::Error> {
        use crate::data::models::schema::authors::dsl::*;

        let mut conn = connect_from_pool().await.map_err(|e| {
            Error::DatabaseError(
                DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string()),
            )
        })?;

        return match authors
            .filter(name.like(format!("%{}%", name_query)))
            .load::<Authors>(&mut conn)
            .await
        {
            Ok(value) => Ok(Some(value)),
            Err(Error::NotFound) => Ok(None),
            Err(e) => Err(e),
        };
    }
}

#[async_trait]
impl Repository for AuthorRepo {
    type Item = Authors;
    type NewItem = NewAuthor<'static>;
    type Form = AuthorForm<'static>;
    type Id = i32;

    async fn get_all(&self) -> Result<Option<Vec<Self::Item>>, result::Error> {
        use crate::data::models::schema::authors::dsl::*;

        let mut conn = connect_from_pool().await.map_err(|e| {
            result::Error::DatabaseError(
                DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string()),
            )
        })?;

        match authors.load::<Self::Item>(&mut conn).await {
            Ok(value) => Ok(Some(value)),
            Err(result::Error::NotFound) => Ok(None),
            Err(e) => Err(e),
        }
    }

    async fn get_by_id(&self, id: Self::Id) -> Result<Option<Self::Item>, result::Error> {
        use crate::data::models::schema::{authors as author, authors::dsl::*};

        let mut conn = connect_from_pool().await.map_err(|e| {
            Error::DatabaseError(
                DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string()),
            )
        })?;

        match authors
            .filter(author::author_id.eq(id))
            .first::<Authors>(&mut conn)
            .await
        {
            Ok(value) => Ok(Some(value)),
            Err(Error::NotFound) => Ok(None),
            Err(e) => Err(e),
        }
    }

    async fn add(&self, new_item: Self::NewItem) -> Result<Self::Item, result::Error> {
        use crate::data::models::schema::authors::dsl::*;

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
                    diesel::insert_into(authors)
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

    async fn update(&self, id: Self::Id, updated_item: Self::Form) -> Result<(), result::Error> {
        use crate::data::models::schema::authors::dsl::*;

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
                    diesel::update(authors.filter(author_id.eq(id)))
                        .set(updated_item)
                        .execute(connection)
                        .await?;

                    Ok(())
                }
                .scope_boxed()
            })
            .await
        {
            Ok(author) => Ok(author),
            Err(e) => Err(e),
        }
    }

    async fn delete(&self, id: Self::Id) -> Result<(), result::Error> {
        use crate::data::models::schema::authors::dsl::*;

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
                    diesel::delete(authors.filter(author_id.eq(id)))
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