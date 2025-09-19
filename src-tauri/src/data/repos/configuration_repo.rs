use diesel::prelude::*;
use diesel::result::*;
use diesel_async::scoped_futures::ScopedFutureExt;
use diesel_async::AsyncConnection;
use diesel_async::RunQueryDsl;

use crate::data::database::*;
use crate::data::models::configuration::Configuration;
use crate::data::models::configuration::ConfigurationForm;

pub async fn get_all_configurations() -> Result<Option<Vec<Configuration>>, Error> {
    use crate::data::models::schema::configuration::dsl::*;

    let mut conn = connect_from_pool().await.map_err(|e| {
        Error::DatabaseError(
            DatabaseErrorKind::UnableToSendCommand,
            Box::new(e.to_string()),
        )
    })?;

    return match configuration
        .select(Configuration::as_select())
        .load(&mut conn)
        .await
    {
        Ok(value) => Ok(Some(value)),
        Err(Error::NotFound) => Ok(None),
        Err(e) => Err(e),
    };
}

pub async fn get_book_path() -> Result<Option<String>, Error> {
    use crate::data::models::schema::configuration::dsl::*;

    let mut conn = connect_from_pool().await.map_err(|e| {
        Error::DatabaseError(
            DatabaseErrorKind::UnableToSendCommand,
            Box::new(e.to_string()),
        )
    })?;

    return match configuration
        .select(book_path)
        .first::<Option<String>>(&mut conn)
        .await
        .optional()
    {
        Ok(Some(value)) => Ok(value),
        Ok(None) => Ok(None),
        Err(e) => Err(e),
    };
}

pub async fn set_book_path(new_path: &str) -> Result<(), Error> {
    use crate::data::models::schema::configuration::dsl::*;

    let new_config: ConfigurationForm<'_> = ConfigurationForm {
        book_path: Some(new_path),
    };

    let pool_conn = connect_from_pool().await;

    let mut conn = match pool_conn {
        Ok(value) => value,
        Err(e) => {
            return Err(Error::DatabaseError(
                DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string()),
            ))
        }
    };

    return match conn
        .transaction(|connection| {
            async move {
                diesel::insert_into(configuration)
                    .values(&new_config)
                    .on_conflict(configuration_id)
                    .do_update()
                    .set(&new_config)
                    .execute(connection)
                    .await?;

                Ok(())
            }
            .scope_boxed()
        })
        .await
    {
        Ok(value) => Ok(value),
        Err(e) => Err(e),
    };
}
