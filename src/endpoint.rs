use serde_json;
use schema::GraphQLHandler;
use db::DatabaseHandler;
use schema::GraphQLRequest;

/// GraphQL global endpoint
#[post("/", format = "application/json", data = "<request>")]
fn graphql_handler(request: GraphQLRequest, conn: DatabaseHandler) -> String {
    // fetch the request query from body
    let query: &str = match request.fetch() {
        Some(q) => q,
        None => ""
    };

    // execute the query against the graphql schema
    let res = GraphQLHandler::execute(query, conn);

    // return response as string
    serde_json::to_string(&res)
        .unwrap_or(String::new())
}