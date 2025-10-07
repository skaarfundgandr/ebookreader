use diesel::prelude::*;
use diesel::result::*;
use diesel_async::scoped_futures::ScopedFutureExt;
use diesel_async::AsyncConnection;
use diesel_async::RunQueryDsl;
use tokio::sync::MutexGuard;

use crate::data::database::*;
use crate::data::models::books::Books;
use crate::data::models::user_library::{NewUserLibrary, UserLibrary};

pub async fn get_all_user_library_entries() -> Result<Option<Vec<UserLibrary>>, Error> {
    use crate::data::models::schema::user_library::dsl::*;

    let mut conn = connect_from_pool().await.map_err(|e| {
        Error::DatabaseError(
            DatabaseErrorKind::UnableToSendCommand,
            Box::new(e.to_string()),
        )
    })?;

    return match user_library.load::<UserLibrary>(&mut conn).await {
        Ok(value) => Ok(Some(value)),
        Err(Error::NotFound) => Ok(None),
        Err(e) => Err(e),
    };
}

pub async fn get_user_library_by_user_id(uid: i32) -> Result<Option<Vec<UserLibrary>>, Error> {
    use crate::data::models::schema::user_library::dsl::*;

    let mut conn = connect_from_pool().await.map_err(|e| {
        Error::DatabaseError(
            DatabaseErrorKind::UnableToSendCommand,
            Box::new(e.to_string()),
        )
    })?;

    return match user_library
        .filter(user_id.eq(uid))
        .load::<UserLibrary>(&mut conn)
        .await
    {
        Ok(value) => Ok(Some(value)),
        Err(Error::NotFound) => Ok(None),
        Err(e) => Err(e),
    };
}

pub async fn get_books_by_user_id(uid: i32) -> Result<Option<Vec<Books>>, Error> {
    use crate::data::models::schema::{books, user_library};

    let mut conn = connect_from_pool().await.map_err(|e| {
        Error::DatabaseError(
            DatabaseErrorKind::UnableToSendCommand,
            Box::new(e.to_string()),
        )
    })?;

    return match user_library::table
        .inner_join(books::table.on(books::book_id.eq(user_library::book_id)))
        .filter(user_library::user_id.eq(uid))
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

pub async fn get_user_library_entry(uid: i32, bid: i32) -> Result<Option<UserLibrary>, Error> {
    use crate::data::models::schema::user_library::dsl::*;

    let mut conn = connect_from_pool().await.map_err(|e| {
        Error::DatabaseError(
            DatabaseErrorKind::UnableToSendCommand,
            Box::new(e.to_string()),
        )
    })?;

    return match user_library
        .filter(user_id.eq(uid).and(book_id.eq(bid)))
        .first::<UserLibrary>(&mut conn)
        .await
    {
        Ok(value) => Ok(Some(value)),
        Err(Error::NotFound) => Ok(None),
        Err(e) => Err(e),
    };
}

pub async fn add_book_to_user_library(new_entry: NewUserLibrary) -> Result<(), Error> {
    use crate::data::models::schema::user_library::dsl::*;

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
                diesel::insert_into(user_library)
                    .values(new_entry)
                    .execute(connection)
                    .await?;

                Ok(())
            }
            .scope_boxed()
        })
        .await
    {
        Ok(()) => Ok(()),
        Err(e) => Err(e),
    };

    return result;
}

pub async fn remove_book_from_user_library(uid: i32, bid: i32) -> Result<(), Error> {
    use crate::data::models::schema::user_library::dsl::*;

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
                diesel::delete(user_library.filter(user_id.eq(uid).and(book_id.eq(bid))))
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

pub async fn remove_all_books_for_user(uid: i32) -> Result<(), Error> {
    use crate::data::models::schema::user_library::dsl::*;

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
                diesel::delete(user_library.filter(user_id.eq(uid)))
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
