use diesel::prelude::*;

use crate::data::models::schema::*;

#[derive(Queryable, Identifiable, Selectable, PartialEq, Debug)]
#[diesel(table_name = users)]
#[diesel(primary_key(user_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Users {
    pub user_id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: String,
}

#[derive(Insertable, PartialEq, Debug)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password_hash: &'a str,
    pub created_at: &'a str,
}