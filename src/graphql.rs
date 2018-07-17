use juniper::{self, Variables, ExecutionError, Value, GraphQLError};
use models::{Product, ProductInput, Country, CountryInput};
use db::{DatabaseHandler};

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

    field allProducts(&executor) -> Vec<Product> {
        let handler = executor.context();
        match handler.get_products() {
            Err(err) => {
                println!("{:?}", err);
                Vec::new()
            },
            Ok(result) => result
        }
    }

    field country(&executor, full_name: String) -> Option<Country> {
        let handler = executor.context();
        handler.get_country_by_id(&full_name).ok()
    }

    field allCountries(&executor) -> Vec<Country> {
        let handler = executor.context();
        match handler.get_countries() {
            Err(err) => {
                println!("{:?}", err);
                Vec::new()
            },
            Ok(result) => result
        }
    }
});

/// Mutation
struct Mutation;

/// Definition of the mutation GraphQL possibilities for the current schema
graphql_object!(Mutation: DatabaseHandler |&self| {

    field apiVersion() -> &str {
        "0.1"
    }

    field createProduct(&executor, products: Vec<ProductInput>) -> Vec<Product> {
        let mut handler = executor.context();
        let proc_products: Vec<Product> = products.iter()
            .map(|pr| pr.to_product())
            .collect();

        match handler.insert_product(&proc_products) {
            Err(err) => {
                println!("{:?}", err);
                Vec::new()
            },
            Ok(result) => result
        }
    }

    field createCountry(&executor, countries: Vec<CountryInput>) -> Vec<Country> {
        let mut handler = executor.context();
        let proc_countries: Vec<Country> = countries.iter()
            .map(|co| co.to_country())
            .collect();

        match handler.insert_country(&proc_countries) {
            Err(err) => {
                println!("{:?}", err);
                Vec::new()
            },
            Ok(result) => result
        }
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