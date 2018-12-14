use std::io::Read;
use serde::Serialize;
use serde_json;
use graphql::GraphQLHandler;
use db::DatabaseHandler;
use rocket::data::{self, FromData, Data, Outcome, Transform};
use rocket::{Request, Outcome::*};
use rocket::http::{Status, ContentType};
use rocket::data::Transformed;

/// GraphQLRequest defines the structure of the request that are sent in to the GraphQL endpoint.
/// Consist of a query or a mutation. Both can not be defined.
#[derive(Serialize, Deserialize, Debug)]
pub struct GraphQLRequest {
    pub query: Option<String>
}

// Implementation of the `FromData` trait from Rocket to read as input data the GraphQLRequest
impl<'a> FromData<'a> for GraphQLRequest {
    type Error = String;
    type Owned = String;
    type Borrowed = str;

    fn transform(_: &Request, data: Data) -> Transform<Outcome<Self::Owned, Self::Error>> {
        let mut json = String::new();
        let outcome = match data.open().read_to_string(&mut json) {
            Ok(_) => Success(json),
            Err(e) => Failure((Status::InternalServerError, format!("{:?}", e)))
        };

        Transform::Borrowed(outcome)
    }

    fn from_data(req: &Request, outcome: Transformed<'a, Self>) -> data::Outcome<Self, Self::Error> {
        if req.content_type() != Some(&ContentType::new("application", "json")) {
            return Failure((Status::UnsupportedMediaType, format!("'JSON' only supported.")));
        }

        let json = outcome.borrowed()?;

        // Extract the graphql request from the body
        match serde_json::from_str(&json) {
            Ok(result) => Success(result),
            Err(e) => Failure((Status::BadRequest, format!("{:?}", e)))
        }
    }
}

// Serialize any type into a JSON string
fn serialize<T: ?Sized>(val: &T) -> String where T: Serialize {
    serde_json::to_string(val)
        .unwrap_or(String::new())
}

/// GraphQL global endpoint
#[post("/graphql", format = "application/json", data = "<request>")]
pub fn graphql_handler(request: GraphQLRequest, conn: DatabaseHandler) -> String {
    let query = request.query.unwrap_or(String::new());

    // execute the query against the graphql schema
    match GraphQLHandler::execute(&query, conn) {
        Ok((val, exec_errors)) => {
            if exec_errors.len() > 0 {
                serialize(&exec_errors)
            } else {
                serialize(&val)
            }
        },
        Err(err) => serialize(&err)
    }
}