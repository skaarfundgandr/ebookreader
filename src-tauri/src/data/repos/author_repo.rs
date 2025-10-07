use diesel::prelude::*;
use diesel::result::*;
use diesel_async::scoped_futures::ScopedFutureExt;
use diesel_async::AsyncConnection;
use diesel_async::RunQueryDsl;
use tokio::sync::MutexGuard;

use crate::data::database::*;
use crate::data::models::authors::{Authors, NewAuthor, UpdateAuthor};

pub async fn get_all_authors() -> Result<Option<Vec<Authors>>, Error> {
    use crate::data::models::schema::authors::dsl::*;

    let mut conn = connect_from_pool().await.map_err(|e| {
        Error::DatabaseError(
            DatabaseErrorKind::UnableToSendCommand,
            Box::new(e.to_string()),
        )
    })?;

    return match authors.load::<Authors>(&mut conn).await {
        Ok(value) => Ok(Some(value)),
        Err(Error::NotFound) => Ok(None),
        Err(e) => Err(e),
    };
}

pub async fn get_author_by_id(id: i32) -> Result<Option<Authors>, Error> {
    use crate::data::models::schema::{authors as author, authors::dsl::*};

    let mut conn = connect_from_pool().await.map_err(|e| {
        Error::DatabaseError(
            DatabaseErrorKind::UnableToSendCommand,
            Box::new(e.to_string()),
        )
    })?;

    return match authors
        .filter(author::author_id.eq(id))
        .first::<Authors>(&mut conn)
        .await
    {
        Ok(value) => Ok(Some(value)),
        Err(Error::NotFound) => Ok(None),
        Err(e) => Err(e),
    };
}

pub async fn get_authors_by_name(author_name: &str) -> Result<Option<Vec<Authors>>, Error> {
    use crate::data::models::schema::authors::dsl::*;

    let mut conn = connect_from_pool().await.map_err(|e| {
        Error::DatabaseError(
            DatabaseErrorKind::UnableToSendCommand,
            Box::new(e.to_string()),
        )
    })?;

    return match authors
        .filter(name.like(format!("%{}%", author_name)))
        .load::<Authors>(&mut conn)
        .await
    {
        Ok(value) => Ok(Some(value)),
        Err(Error::NotFound) => Ok(None),
        Err(e) => Err(e),
    };
}

pub async fn get_authors_by_book_id(book_id_param: i32) -> Result<Option<Vec<Authors>>, Error> {
    use crate::data::models::schema::{authors, book_authors};

    let mut conn = connect_from_pool().await.map_err(|e| {
        Error::DatabaseError(
            DatabaseErrorKind::UnableToSendCommand,
            Box::new(e.to_string()),
        )
    })?;

    return match book_authors::table
        .inner_join(authors::table.on(authors::author_id.eq(book_authors::author_id)))
        .filter(book_authors::book_id.eq(book_id_param))
        .select((authors::author_id, authors::name))
        .load::<Authors>(&mut conn)
        .await
    {
        Ok(value) => Ok(Some(value)),
        Err(Error::NotFound) => Ok(None),
        Err(e) => Err(e),
    };
}

pub async fn create_author(new_author: NewAuthor<'_>) -> Result<(), Error> {
    use crate::data::models::schema::authors::dsl::*;

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
                diesel::insert_into(authors)
                    .values(new_author)
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

pub async fn update_author(id: i32, author_update: UpdateAuthor<'_>) -> Result<(), Error> {
    use crate::data::models::schema::authors::dsl::*;

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
                diesel::update(authors.filter(author_id.eq(id)))
                    .set(author_update)
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
    };

    return result;
}

pub async fn delete_author(id: i32) -> Result<(), Error> {
    use crate::data::models::schema::authors::dsl::*;

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
    };

    return result;
}
