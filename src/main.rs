#![warn(missing_docs)]
/// BookScan
mod types;
mod db;
mod routes;

use types::*;
use db::BsDb;
use juniper::{graphql_object, EmptyMutation, EmptySubscription, FieldResult, GraphQLEnum, GraphQLInputObject, GraphQLObject, ScalarValue};
use rocket::{routes};
use sqlx::{Connection};
use sqlx::postgres::PgPoolOptions;


struct Query;

#[graphql_object]
#[graphql(context = Context)]
impl Query {
    async fn api_version() -> &'static str {
        "1.0"
    }

    async fn student_id(id: i32, context: &Context, ) -> FieldResult<Option<ExpandedStudent>> {
        println!("Query for user with id: {}", id);
        let student = context.db.get_student_id(id).await?;
        if student.is_none() {
            Ok(None)
        } else {
            Ok(Some(context.db.get_expanded_student(&student.unwrap()).await?))
        }
    }

    async fn student_name(first_name: String, last_name: String, context: &Context, ) -> FieldResult<Option<ExpandedStudent>> {
        println!("Query for student with first_name: {}, last_name: {}", first_name, last_name);
        let student = context.db.get_student_name(&first_name, &last_name).await?;

        if student.is_none() {
            Ok(None)
        } else {
            Ok(Some(context.db.get_expanded_student(&student.unwrap()).await?))
        }
    }
}



/// Entrypoint
#[rocket::main]
async fn main()  {
    // Open a connection pool to the BookScan database
    let conn = match PgPoolOptions::new()
        .max_connections(2)
        .connect("postgres://postgres:1234@localhost/postgres").await {
        Ok(c) => c,
        Err(_) => { !panic!() }
    };

    // Create context for handling GraphQL requests
    let ctx = Context { db: BsDb::new(conn) };

    // Start Rocket webserver
    _ = rocket::build()
        .manage(ctx)
        .manage(Schema::new(Query, EmptyMutation::<Context>::new(), EmptySubscription::<Context>::new()))
        .mount("/", routes![routes::graphql::graphiql, routes::graphql::playground, routes::graphql::get_graphql, routes::graphql::post_graphql])
        .launch()
        .await
        .expect("failed to launch rocket");
}
