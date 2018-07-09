use std::io::Read;
use serde_json;
use graphql::GraphQLHandler;
use db::DatabaseHandler;
use rocket::data::{self, FromData, Data};
use rocket::{Outcome, Request};
use rocket::http::{Status, ContentType};

/// GraphQLRequest defines the structure of the request that are sent in to the GraphQL endpoint.
/// Consist of a query or a mutation. Both can not be defined.
#[derive(Serialize, Deserialize, Debug)]
pub struct GraphQLRequest {
    pub query: Option<String>
}

// Implementation of the `FromData` trait from Rocket to read as input data the GraphQLRequest
impl FromData for GraphQLRequest {
    type Error = String;
    fn from_data(req: &Request, data: Data) -> data::Outcome<Self, String> {
        // Data has content type "application/json"
        if req.content_type() != Some(&ContentType::new("application", "json")) {
            return Outcome::Forward(data);
        }

        // Read the data into a String.
        let mut json = String::new();
        if let Err(e) = data.open().read_to_string(&mut json) {
            return Outcome::Failure((Status::InternalServerError, format!("{:?}", e)));
        }

        // Extract the graphql request from the body
        let graphql_request: GraphQLRequest;
        match serde_json::from_str(&json) {
            Ok(result) => graphql_request = result,
            Err(e) => {
                return Outcome::Failure((Status::BadRequest, format!("{:?}", e)));
            }
        };

        Outcome::Success(graphql_request)
    }
}

/// GraphQL global endpoint
#[post("/graphql", format = "application/json", data = "<request>")]
fn graphql_handler(request: GraphQLRequest, conn: DatabaseHandler) -> String {
    let query = request.query.unwrap_or(String::new());

    // execute the query against the graphql schema
    let res = GraphQLHandler::execute(&query, conn);

    // return response as string
    serde_json::to_string(&res)
        .unwrap_or(String::new())
}