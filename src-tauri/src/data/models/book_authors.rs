use diesel::prelude::*;

use crate::data::models::schema::*;
use crate::data::models::books::Books;
use crate::data::models::authors::Authors;

#[derive(Queryable, Identifiable, Associations, PartialEq, Insertable,Debug)]
#[diesel(table_name = book_authors)]
#[diesel(primary_key(book_id, author_id))]
#[diesel(belongs_to(Books, foreign_key = book_id))]
#[diesel(belongs_to(Authors, foreign_key = author_id))]
pub struct BookAuthors {
    pub book_id: i32,
    pub author_id: i32,
}