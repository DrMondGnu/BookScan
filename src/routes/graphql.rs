use rocket::response::content::RawHtml;
use rocket::State;
use crate::Schema;
use crate::types::Context;

#[rocket::get("/playground")]
pub async fn playground() -> RawHtml<String> {
    juniper_rocket::playground_source("/graphql", None)
}

#[rocket::post("/graphql", data = "<request>")]
pub async fn post_graphql(
    db: &State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(schema, db).await
}

#[rocket::get("/graphql?<request..>")]
pub async fn get_graphql(
    db: &State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(schema, db).await
}

#[rocket::get("/graphiql")]
pub async fn graphiql() -> RawHtml<String> {
    juniper_rocket::graphiql_source("/graphql", None)
}