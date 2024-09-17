use core::convert::TryFrom;
use juniper::{GraphQLEnum, GraphQLObject, GraphQLScalar};



#[derive(sqlx::Type, Debug, GraphQLEnum)]
pub enum Subject {
    Math,
    German,
    History,
    Art,
}

#[derive(sqlx::FromRow, GraphQLObject)]
pub struct Book {
    pub id: i32,
    pub name: String,
    pub subject: Subject,
}

#[derive(sqlx::FromRow, GraphQLObject)]
pub struct Student {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}
#[derive(GraphQLObject)]
pub struct ExpandedBook {
    id: i32,
    name: String,
    subject: Subject,
    owner: Student,
}
#[derive(GraphQLObject)]
pub struct ExpandedStudent {
    id: i32,
    first_name: String,
    last_name: String,
    books: Option<Vec<Book>>,
}
impl ExpandedStudent {
    pub fn new(student: &Student, books: Option<Vec<Book>>) -> Self {
        Self { id: student.id, first_name: student.first_name.clone(), last_name: student.last_name.clone(), books }
    }
}