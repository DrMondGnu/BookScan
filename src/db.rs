use std::future::Future;
use juniper::futures::future::AndThen;
use rocket::futures::TryFutureExt;
use rocket::sentinel::resolution::Resolve;
use sqlx::{Error, PgPool};
use sqlx::postgres::PgStatement;
use crate::types::{Book, ExpandedBook, ExpandedStudent, Student};

pub struct BsDb {
    db: PgPool,
    //stmt_student_id: PgStatement<'a>,
}

/// Interacts with BookScan database
impl BsDb {

    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
    /// Queries for a student
    pub fn get_student_id(&self, id: i32) -> impl Future<Output=Result<Option<Student>, Error>> + '_ {
        sqlx::query_as::<_, Student>("SELECT * FROM students WHERE id = $1 LIMIT 1")
            .bind(id)
            .fetch_optional(&self.db)
    }
    pub fn get_student_name<'a>(&'a self, first_name: &'a str, last_name: &'a str) -> impl Future<Output=Result<Option<Student>, Error>> + '_{
        sqlx::query_as::<_, Student>("SELECT * FROM students WHERE first_name = $1 AND last_name = $2 LIMIT 1")
            .bind(first_name)
            .bind(last_name)
            .fetch_optional(&self.db)
    }

    pub fn get_students_books<'a>(&'a self, student: &'a Student) -> impl Future<Output=Result<Vec<Book>, Error>> + 'a  {
        sqlx::query_as::<_, Book>("SELECT Books.* FROM Books JOIN StudentsBooks ON Books.id = StudentsBooks.book_id JOIN Students ON Students.id = StudentsBooks.student_id WHERE Students.id = $1;")
            .bind(student.id)
            .fetch_all(&self.db)
    }

     pub fn get_book_id(&self, id: i32) -> impl Future<Output=Result<Option<Book>, Error>> + '_ {
         sqlx::query_as::<_, Book>("SELECT * FROM books WHERE id = $1 LIMIT 1")
             .bind(id)
             .fetch_optional(&self.db)
     }

    pub fn get_book_owner<'a>(&'a self, book: &'a Book)  -> impl Future<Output=Result<Option<Student>, Error>> + 'a  {
        sqlx::query_as::<_, Student>(
        "SELECT Students.*
            FROM Students
            JOIN StudentsBooks ON Students.id = StudentsBooks.student_id
            JOIN Books ON Books.id = StudentsBooks.book_id
            WHERE Books.id = $1;
            ")
            .bind(book.id)
            .fetch_optional(&self.db)
    }
    pub async fn get_expanded_book(&self, book: &Book) -> Result<ExpandedBook, Error> {
        let owner = self.get_book_owner(book).await?;
        if owner.is_none() {
            Ok(ExpandedBook::new(book, None))
        } else {
            Ok(ExpandedBook::new(book, owner))
        }
    }

    pub async fn get_expanded_student(&self, student: &Student) -> Result<ExpandedStudent, Error> {
        let books = self.get_students_books(student).await?;
        if books.is_empty() {
            Ok(ExpandedStudent::new(student, None))
        } else {
            Ok(ExpandedStudent::new(student, Some(books)))
        }
    }
}