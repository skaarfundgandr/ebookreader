use diesel::prelude::*;

use crate::data::models::schema::*;

#[derive(Queryable, Selectable, Debug, PartialEq, Clone)]
#[diesel(table_name = libraries)]
#[diesel(primary_key(library_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Library {
    pub library_id: i32,
    pub name: String,
    pub path: String,
    pub added_by: Option<i32>,
    pub added_at: Option<String>,
}

#[derive(Insertable, PartialEq, Debug)]
#[diesel(table_name = libraries)]
pub struct NewLibrary<'a> {
    pub name: &'a str,
    pub path: &'a str,
    pub added_by: Option<i32>,
}

#[derive(AsChangeset, PartialEq, Debug)]
#[diesel(table_name = libraries)]
pub struct UpdateLibrary<'a> {
    pub name: Option<&'a str>,
    pub path: Option<&'a str>,
    pub added_by: Option<i32>,
}
