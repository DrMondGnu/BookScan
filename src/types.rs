use core::convert::TryFrom;
use juniper::{EmptyMutation, EmptySubscription, GraphQLEnum, GraphQLObject, GraphQLScalar, RootNode};
use rocket::yansi::Paint;
use crate::db::BsDb;
use crate::Query;

#[derive(sqlx::Type, Debug, GraphQLEnum, Clone, Copy)]
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

#[derive(sqlx::FromRow, GraphQLObject, Clone, Debug)]
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
    owner: Option<Student>,
}
impl ExpandedBook {
    pub fn new(book: &Book, student: Option<Student>) -> Self {
        Self { id: book.id, name: book.name.clone(), subject: book.subject.clone(), owner: student}
    }
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

/// Context struct for GraphQL
pub struct Context {
    pub db: BsDb,
}
impl juniper::Context for Context {}

pub type Schema = RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;
