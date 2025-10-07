use diesel::prelude::*;
use diesel::result::*;
use diesel_async::scoped_futures::ScopedFutureExt;
use diesel_async::AsyncConnection;
use diesel_async::RunQueryDsl;
use tokio::sync::MutexGuard;

use crate::data::database::*;
use crate::data::models::publishers::{NewPublisher, Publishers, UpdatePublisher};

pub async fn get_all_publishers() -> Result<Option<Vec<Publishers>>, Error> {
    use crate::data::models::schema::publishers::dsl::*;

    let mut conn = connect_from_pool().await.map_err(|e| {
        Error::DatabaseError(
            DatabaseErrorKind::UnableToSendCommand,
            Box::new(e.to_string()),
        )
    })?;

    return match publishers.load::<Publishers>(&mut conn).await {
        Ok(value) => Ok(Some(value)),
        Err(Error::NotFound) => Ok(None),
        Err(e) => Err(e),
    };
}

pub async fn get_publisher_by_id(id: i32) -> Result<Option<Publishers>, Error> {
    use crate::data::models::schema::{publishers as publisher, publishers::dsl::*};

    let mut conn = connect_from_pool().await.map_err(|e| {
        Error::DatabaseError(
            DatabaseErrorKind::UnableToSendCommand,
            Box::new(e.to_string()),
        )
    })?;

    return match publishers
        .filter(publisher::publisher_id.eq(id))
        .first::<Publishers>(&mut conn)
        .await
    {
        Ok(value) => Ok(Some(value)),
        Err(Error::NotFound) => Ok(None),
        Err(e) => Err(e),
    };
}

pub async fn get_publishers_by_name(
    publisher_name: &str,
) -> Result<Option<Vec<Publishers>>, Error> {
    use crate::data::models::schema::publishers::dsl::*;

    let mut conn = connect_from_pool().await.map_err(|e| {
        Error::DatabaseError(
            DatabaseErrorKind::UnableToSendCommand,
            Box::new(e.to_string()),
        )
    })?;

    return match publishers
        .filter(name.like(format!("%{}%", publisher_name)))
        .load::<Publishers>(&mut conn)
        .await
    {
        Ok(value) => Ok(Some(value)),
        Err(Error::NotFound) => Ok(None),
        Err(e) => Err(e),
    };
}

pub async fn create_publisher(new_publisher: NewPublisher<'_>) -> Result<(), Error> {
    use crate::data::models::schema::publishers::dsl::*;

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
                diesel::insert_into(publishers)
                    .values(new_publisher)
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

pub async fn update_publisher(id: i32, publisher_update: UpdatePublisher<'_>) -> Result<(), Error> {
    use crate::data::models::schema::publishers::dsl::*;

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
                diesel::update(publishers.filter(publisher_id.eq(id)))
                    .set(publisher_update)
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

pub async fn delete_publisher(id: i32) -> Result<(), Error> {
    use crate::data::models::schema::publishers::dsl::*;

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
    };

    return result;
}
