use diesel::prelude::*;
use crate::data::models::schema::*;

#[derive(Queryable, Selectable, Identifiable, PartialEq, Debug)]
#[diesel(table_name = configuration)]
#[diesel(primary_key(configuration_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Configuration {
    pub configuration_id: i32,
    pub book_path: Option<String>
}

#[derive(Debug, AsChangeset, Insertable)]
#[diesel(table_name = configuration)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ConfigurationForm<'a> {
    pub book_path: Option<&'a str>
}