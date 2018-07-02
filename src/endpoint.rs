use serde_json;
use schema::GraphQLHandler;
use db::DatabaseHandler;
use schema::GraphQLRequest;

#[post("/", format = "application/json", data = "<request>")]
fn graphql_handler(request: GraphQLRequest, conn: DatabaseHandler) -> String {
    let query: &str = match request.fetch() {
        Some(q) => q,
        None => ""
    };
    let res = GraphQLHandler::execute(query, conn);
    serde_json::to_string(&res).unwrap()
}           