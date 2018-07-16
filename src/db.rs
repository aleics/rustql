use diesel::{self, PgConnection, prelude::*};
use r2d2_diesel::ConnectionManager;
use r2d2::{Pool, PooledConnection};

use models::Product;
use error::Error;

use rocket::http::Status;
use rocket::{Request, State, Outcome};
use rocket::request::{self, FromRequest};

fn format_error(msg: &'static str, diesel_error: diesel::result::Error) -> Error {
    Error::db(&format!("{}: {}", msg, diesel_error))
}

/// DatabaseHandler handles a single connection to the database
pub struct DatabaseHandler {
    conn: PooledConnection<ConnectionManager<PgConnection>>
}

impl DatabaseHandler {

    fn conn(&self) -> &PgConnection {
        &*self.conn
    }

    /// Read a product from the database for the given UUID
    pub fn get_product_by_id(&self, product_id: &String) -> Result<Product, Error> {
        use schema::products::dsl::*;

        products.find(product_id).first(self.conn())
            .map_err(|err: diesel::result::Error| format_error("could not select product by id", err))
    }

    pub fn get_products(&self) -> Result<Vec<Product>, Error> {
        use schema::products::dsl::*;

        products
            .select((id, name, price, description, country))
            .load::<Product>(self.conn())
            .map_err(|err: diesel::result::Error| format_error("could not select all products", err))
    }

    // Insert a product for a given UUID
    pub fn insert_product(&self, product: &Product) -> Result<Vec<Product>, Error> {
        use schema::products::dsl::*;

        if let Err(err) = diesel::insert_into(products)
            .values(vec![product])
            .execute(self.conn()) {
            return Err(format_error("could not insert product", err));
        }

        self.get_products()
    }
}

/// Implementation of the `FromRequest` rocket trait for the DatabaseHandler struct.
/// This will allow us to retrieve a connection of the database dynamically for a given request.
impl<'a, 'r> FromRequest<'a, 'r> for DatabaseHandler {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let pool = request.guard::<State<Database>>()?;
        match pool.handler() {
            Ok(handler) => Outcome::Success(handler),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

// Database manages the postgres database pool to retrieve and read connections
pub struct Database {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl Database {
    /// Initialization of the database pool
    pub fn init(db_url: &'static str) -> Database {
        let manager = ConnectionManager::<PgConnection>::new(db_url);

        Database {
            pool: Pool::builder().build(manager).expect("unable to generate initial database pool")
        }
    }

    /// Get a database handler for a given connection
    pub fn handler(&self) -> Result<DatabaseHandler, Error> {
        match self.pool.get() {
            Ok(conn) => Ok(DatabaseHandler { conn }),
            Err(_) => Err(Error::db("can not retrieve a connection from pool"))
        }

    }
}