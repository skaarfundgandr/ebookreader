use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::data::database::*;
use crate::data::models::configuration::Configuration;

pub async fn get_all_configuration() -> Result<Option<Vec<Configuration>>, diesel::result::Error> {
    use crate::data::models::schema::configuration::dsl::*;

    let mut conn = match connect_from_pool().await {
        Ok(value) => value,
        Err(_) => panic!("Failed to connect from pool"),
    };

    return match configuration
        .select(Configuration::as_select())
        .load(&mut conn)
        .await {
            Ok(value) => Ok(Some(value)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(e),
        };
}