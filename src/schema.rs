use std::io::Read;
use uuid::{Uuid, UuidVersion};
use serde_json;
use juniper::{self, EmptyMutation, Variables, ExecutionError, Value, GraphQLError};
use db::{DatabaseHandler};
use rocket::data::{self, FromData, Data};
use rocket::{Outcome, Request};
use rocket::http::{Status, ContentType};

#[derive(Clone, GraphQLObject)]
#[graphql(description="Product structure")]
pub struct Product {
    pub id: String,
    pub name: String,
    pub price: f64,
    pub description: String,
    pub available: bool,
}

#[derive(Clone, GraphQLInputObject)]
#[graphql(description="Input product structure")]
pub struct NewProduct {
    name: String,
    description: String,
    price: f64,
    available: bool,
}

impl NewProduct {
    fn to_product(&self) -> Product {
        let uuid = Uuid::new(UuidVersion::Sha1).unwrap().to_string();

        Product {
            id: uuid,
            name: self.name.clone(),
            price: self.price.clone(),
            description: self.description.clone(),
            available: self.available.clone()
        }
    }
}


#[derive(Clone, GraphQLObject)]
#[graphql(description="Country structure")]
pub struct Country {
    full_name: String,
    continent: String,
    short_name: String,
}

#[derive(Clone, GraphQLInputObject)]
#[graphql(description="Country structure")]
pub struct NewCountry {
    full_name: String,
    continent: String,
    short_name: String,
}

impl NewCountry {
    fn to_country(&self) -> Country {
        Country {
            full_name: self.full_name.clone(),
            continent: self.continent.clone(),
            short_name: self.short_name.clone()
        }
    }
}


impl juniper::Context for DatabaseHandler {}

struct Query;

graphql_object!(Query: DatabaseHandler |&self| {

    field apiVersion() -> &str {
        "0.1"
    }

    field product(&executor, id: String) -> Option<Product> {
        let handler = executor.context();
        handler.get_product_by_id(&id).ok()
    }
});

struct Mutation;

graphql_object!(Mutation: DatabaseHandler |&self| {

    field apiVersion() -> &str {
        "0.1"
    }

    field product(&executor, new_product: NewProduct) -> Option<Vec<Product>> {
        let mut handler = executor.context();
        let product = new_product.to_product();

        handler.insert_product_by_id(&product.id, &product).ok()
    }
});


type Schema = juniper::RootNode<'static, Query, EmptyMutation<DatabaseHandler>>;

pub struct GraphQLHandler;

impl<'a> GraphQLHandler {
    pub fn execute(query: &'a str, conn: DatabaseHandler) -> Result<(Value, Vec<ExecutionError>), GraphQLError<'a>> {
        juniper::execute(
            query,
            None,
            &Schema::new(Query, EmptyMutation::new()),
            &Variables::new(),
            &conn,
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct GraphQLRequest {
    pub query: Option<String>,
    pub mutation: Option<String>
}

impl GraphQLRequest {
    pub fn fetch(&self) -> &Option<String> {
        match self.query.is_some() {
            true => &self.query,
            false => match self.mutation.is_some() {
                true => &self.mutation,
                false => &None
            }
        }
    }
}

impl FromData for GraphQLRequest {
    type Error = String;
    fn from_data(req: &Request, data: Data) -> data::Outcome<Self, String> {
        if req.content_type() != Some(&ContentType::new("application", "json")) {
            return Outcome::Forward(data);
        }

        // Read the data into a String.
        let mut json = String::new();
        if let Err(e) = data.open().read_to_string(&mut json) {
            return Outcome::Failure((Status::InternalServerError, format!("{:?}", e)));
        }

        let graphql_request: GraphQLRequest;
        match serde_json::from_str(&json) {
            Ok(result) => { graphql_request = result; },
            Err(e) => {
                return Outcome::Failure((Status::BadRequest, format!("{:?}", e)));
            }
        };

        Outcome::Success(graphql_request)
    }
}