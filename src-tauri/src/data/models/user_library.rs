use diesel::prelude::*;

use crate::data::models::schema::*;
use crate::data::models::users::Users;
use crate::data::models::books::Books;

#[derive(Queryable, Identifiable, Associations, PartialEq, Debug)]
#[diesel(table_name = user_library)]
#[diesel(primary_key(user_id, book_id))]
#[diesel(belongs_to(Users, foreign_key = user_id))]
#[diesel(belongs_to(Books, foreign_key = book_id))]
pub struct UserLibrary {
    pub user_id: i32,
    pub book_id: i32,
    pub added_at: Option<String>,
}

#[derive(Insertable, PartialEq, Debug)]
#[diesel(table_name = user_library)]
pub struct NewUserLibrary {
    pub user_id: i32,
    pub book_id: i32,
}