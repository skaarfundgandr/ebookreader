use diesel::prelude::*;
use diesel::result::*;
use diesel_async::scoped_futures::ScopedFutureExt;
use diesel_async::AsyncConnection;
use diesel_async::RunQueryDsl;
use tokio::sync::MutexGuard;

use crate::data::database::*;
use crate::data::models::authors::Authors;
use crate::data::models::book_authors::BookAuthors;
use crate::data::models::books::Books;
// TODO: Make this implement the Repository trait and deprecate the functions in book_author_repo.rs after
pub async fn get_all_book_author_relationships() -> Result<Option<Vec<BookAuthors>>, Error> {
    use crate::data::models::schema::book_authors::dsl::*;

    let mut conn = connect_from_pool().await.map_err(|e| {
        Error::DatabaseError(
            DatabaseErrorKind::UnableToSendCommand,
            Box::new(e.to_string()),
        )
    })?;

    return match book_authors.load::<BookAuthors>(&mut conn).await {
        Ok(value) => Ok(Some(value)),
        Err(Error::NotFound) => Ok(None),
        Err(e) => Err(e),
    };
}

pub async fn get_authors_by_book_id(bid: i32) -> Result<Option<Vec<Authors>>, Error> {
    use crate::data::models::schema::{authors, book_authors};

    let mut conn = connect_from_pool().await.map_err(|e| {
        Error::DatabaseError(
            DatabaseErrorKind::UnableToSendCommand,
            Box::new(e.to_string()),
        )
    })?;

    return match book_authors::table
        .inner_join(authors::table.on(authors::author_id.eq(book_authors::author_id)))
        .filter(book_authors::book_id.eq(bid))
        .select((authors::author_id, authors::name))
        .load::<Authors>(&mut conn)
        .await
    {
        Ok(value) => Ok(Some(value)),
        Err(Error::NotFound) => Ok(None),
        Err(e) => Err(e),
    };
}

pub async fn get_books_by_author_id(aid: i32) -> Result<Option<Vec<Books>>, Error> {
    use crate::data::models::schema::{book_authors, books};

    let mut conn = connect_from_pool().await.map_err(|e| {
        Error::DatabaseError(
            DatabaseErrorKind::UnableToSendCommand,
            Box::new(e.to_string()),
        )
    })?;

    return match book_authors::table
        .inner_join(books::table.on(books::book_id.eq(book_authors::book_id)))
        .filter(book_authors::author_id.eq(aid))
        .select((
            books::book_id,
            books::title,
            books::published_date,
            books::publisher_id,
            books::isbn,
            books::file_type,
            books::file_path,
            books::added_at,
        ))
        .load::<Books>(&mut conn)
        .await
    {
        Ok(value) => Ok(Some(value)),
        Err(Error::NotFound) => Ok(None),
        Err(e) => Err(e),
    };
}

pub async fn get_book_author_relationship(
    bid: i32,
    aid: i32,
) -> Result<Option<BookAuthors>, Error> {
    use crate::data::models::schema::book_authors::dsl::*;

    let mut conn = connect_from_pool().await.map_err(|e| {
        Error::DatabaseError(
            DatabaseErrorKind::UnableToSendCommand,
            Box::new(e.to_string()),
        )
    })?;

    return match book_authors
        .filter(book_id.eq(bid).and(author_id.eq(aid)))
        .first::<BookAuthors>(&mut conn)
        .await
    {
        Ok(value) => Ok(Some(value)),
        Err(Error::NotFound) => Ok(None),
        Err(e) => Err(e),
    };
}

pub async fn add_author_to_book(bid: i32, aid: i32) -> Result<(), Error> {
    use crate::data::models::schema::book_authors::dsl::*;

    let mut conn = connect_from_pool().await.map_err(|e| {
        Error::DatabaseError(
            DatabaseErrorKind::UnableToSendCommand,
            Box::new(e.to_string()),
        )
    })?;

    let db_lock = lock_db();
    let _guard: MutexGuard<()> = db_lock.lock().await;

    let new_relationship = BookAuthors {
        book_id: bid,
        author_id: aid,
    };

    let result = match conn
        .transaction(|connection| {
            async move {
                diesel::insert_into(book_authors)
                    .values(new_relationship)
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

pub async fn remove_author_from_book(bid: i32, aid: i32) -> Result<(), Error> {
    use crate::data::models::schema::book_authors::dsl::*;

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
                diesel::delete(book_authors.filter(book_id.eq(bid).and(author_id.eq(aid))))
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

pub async fn remove_all_authors_from_book(bid: i32) -> Result<(), Error> {
    use crate::data::models::schema::book_authors::dsl::*;

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
                diesel::delete(book_authors.filter(book_id.eq(bid)))
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

pub async fn remove_all_books_from_author(aid: i32) -> Result<(), Error> {
    use crate::data::models::schema::book_authors::dsl::*;

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
                diesel::delete(book_authors.filter(author_id.eq(aid)))
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
