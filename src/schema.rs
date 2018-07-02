use uuid::{Uuid, UuidVersion};
use juniper::{self, Variables, ExecutionError, Value, GraphQLError};
use db::{DatabaseHandler};

/// Product schema structure
#[derive(Clone, GraphQLObject)]
#[graphql(description="Product structure")]
pub struct Product {
    pub id: String,
    pub name: String,
    pub price: f64,
    pub description: String,
    pub available: bool,
}

/// Product schema structure for input data (mutations)
#[derive(Clone, GraphQLInputObject)]
#[graphql(description="Input product structure")]
pub struct ProductInput {
    name: String,
    description: String,
    price: f64,
    available: bool,
}

impl ProductInput {
    /// Generate a Product instance with a UUID from an input product
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


/// Country schema structure
#[derive(Clone, GraphQLObject)]
#[graphql(description="Country structure")]
pub struct Country {
    full_name: String,
    continent: String,
    short_name: String,
}

/// Country schema structure for input data (mutations)
#[derive(Clone, GraphQLInputObject)]
#[graphql(description="Country structure")]
pub struct CountryInput {
    full_name: String,
    continent: String,
    short_name: String,
}

impl CountryInput {
    /// Generate a country object from a country input
    fn _to_country(&self) -> Country {
        Country {
            full_name: self.full_name.clone(),
            continent: self.continent.clone(),
            short_name: self.short_name.clone()
        }
    }
}

/// Use the database handler as a context for the graphql
impl juniper::Context for DatabaseHandler {}

/// Query
struct Query;

/// Definition of the query GraphQL possibilities for the current schema
graphql_object!(Query: DatabaseHandler |&self| {

    field apiVersion() -> &str {
        "0.1"
    }

    field product(&executor, id: String) -> Option<Product> {
        let handler = executor.context();
        handler.get_product_by_id(&id).ok()
    }
});

/// Mutation
struct Mutation;

/// Definition of the mutation GraphQL possibilities for the current schema
graphql_object!(Mutation: DatabaseHandler |&self| {

    field apiVersion() -> &str {
        "0.1"
    }

    field product(&executor, new_product: ProductInput) -> Option<Vec<Product>> {
        let mut handler = executor.context();
        let product = new_product.to_product();

        handler.insert_product_by_id(&product).ok()
    }
});

/// Schema definition for the given query and mutation structure
type Schema = juniper::RootNode<'static, Query, Mutation>;

/// GraphQLHandler handles any request delivered to the endpoint and returns the data by a given
/// query and a database connection
pub struct GraphQLHandler;

impl<'a> GraphQLHandler {
    pub fn execute(query: &'a str, conn: DatabaseHandler) -> Result<(Value, Vec<ExecutionError>), GraphQLError<'a>> {
        juniper::execute(
            query,
            None,
            &Schema::new(Query, Mutation),
            &Variables::new(),
            &conn,
        )
    }
}