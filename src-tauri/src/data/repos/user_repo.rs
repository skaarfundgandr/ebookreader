use diesel::prelude::*;
use diesel::result::*;
use diesel_async::RunQueryDsl;

use crate::data::database::*;
use crate::data::models::users::Users;

pub async fn get_all_users() -> Result<Option<Vec<Users>>, Error> {
    use crate::data::models::schema::users::dsl::*;

    let mut conn = connect_from_pool()
        .await
        .map_err(|e| {
            Error::DatabaseError (
                DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string())
            )
        })?;

    return match users
        .select(Users::as_select())
        .load(&mut conn)
        .await {
            Ok(value) => Ok(Some(value)),
            Err(Error::NotFound) => Ok(None),
            Err(e) => Err(e),
        };
}

//TODO: Add more user related database operations