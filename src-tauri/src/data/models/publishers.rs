use diesel::prelude::*;

use crate::data::models::schema::*;

#[derive(Queryable, Identifiable, Selectable, PartialEq, Debug)]
#[diesel(table_name = publishers)]
#[diesel(primary_key(publisher_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Publishers {
    pub publisher_id: i32,
    pub name: String,
}

#[derive(Insertable, PartialEq, Debug)]
#[diesel(table_name = publishers)]
pub struct NewPublisher<'a> {
    pub name: &'a str,
}

#[derive(AsChangeset, PartialEq, Debug)]
#[diesel(table_name = publishers)]
pub struct UpdatePublisher<'a> {
    pub name: Option<&'a str>,
}