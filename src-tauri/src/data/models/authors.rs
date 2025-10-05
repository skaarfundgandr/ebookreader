use diesel::prelude::*;

use crate::data::models::schema::*;

#[derive(Queryable, Identifiable, Selectable, PartialEq, Debug)]
#[diesel(table_name = authors)]
#[diesel(primary_key(author_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Authors {
    pub author_id: i32,
    pub name: String,
}

#[derive(Insertable, PartialEq, Debug)]
#[diesel(table_name = authors)]
pub struct NewAuthor<'a> {
    pub name: &'a str,
}

#[derive(AsChangeset, PartialEq, Debug)]
#[diesel(table_name = authors)]
pub struct UpdateAuthor<'a> {
    pub name: Option<&'a str>,
}
