mod types;
mod db;

use types::*;
use std::thread::panicking;
use juniper::{graphql_object, EmptyMutation, EmptySubscription, FieldResult, GraphQLEnum, GraphQLInputObject, GraphQLObject, RootNode, ScalarValue};
use rocket::{get, launch, routes, State};
use rocket::form::Strict;
use rocket::form::validate::contains;
use rocket::response::content::RawHtml;
use sqlx::{Connection, Error, PgConnection, PgPool};
use sqlx::postgres::PgPoolOptions;
use crate::db::BsDb;

struct Context {
    db: BsDb,
}
impl juniper::Context for Context {}

struct Query;


#[graphql_object]
#[graphql(context = Context)]
impl Query {
    fn api_version() -> &'static str {
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
type Schema = RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

#[rocket::get("/playground")]
fn playground() -> RawHtml<String> {
    juniper_rocket::playground_source("/graphql", None)
}

#[rocket::post("/graphql", data = "<request>")]
async fn post_graphql(
    db: &State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(schema, db).await
}

#[rocket::get("/graphql?<request..>")]
async fn get_graphql(
    db: &State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(schema, db).await
}

#[rocket::get("/graphiql")]
fn graphiql() -> RawHtml<String> {
    juniper_rocket::graphiql_source("/graphql", None)
}

/// Entrypoint
#[rocket::main]
async fn main()  {

    let mut conn = match PgPoolOptions::new()
        .max_connections(2)
        .connect("postgres://postgres:1234@localhost/postgres").await {
        Ok(c) => c,
        Err(_) => { !panic!() }
    };
    let ctx = Context { db: BsDb::new(conn) };

    _ = rocket::build()
        .manage(ctx)
        .manage(Schema::new(Query, EmptyMutation::<Context>::new(), EmptySubscription::<Context>::new()))
        .mount("/", routes![graphiql, playground, get_graphql, post_graphql])
        .launch()
        .await
        .expect("failed to launch rocket");
}
