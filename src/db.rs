use diesel::{PgConnection, Connection};
use r2d2_diesel::ConnectionManager;
use r2d2::{Pool, PooledConnection};

use models::Product;
use error::Error;

use rocket::http::Status;
use rocket::{Request, State, Outcome};
use rocket::request::{self, FromRequest};

/// Definition of multiple database query as constants
const CREATE_PRODUCTS_TABLE: &'static str = "CREATE TABLE IF NOT EXISTS products (\
    id varchar(100) primary key,\
    name varchar(100) NOT NULL,\
    price double precision NOT NULL,\
    description varchar(250),\
    available boolean NOT NULL\
);";
const SELECT_PRODUCTS: &'static str = "SELECT * FROM products;";
const SELECT_PRODUCT_BY_ID: &'static str = "SELECT * FROM products WHERE id = $1;";
const INSERT_PRODUCT: &'static str = "INSERT INTO products (id, name, price, description, available)\
    VALUES ($1, $2, $3, $4, $5);";

/// DatabaseHandler handles a single connection to the database
pub struct DatabaseHandler {
    conn: PooledConnection<ConnectionManager<PgConnection>>,
}

impl DatabaseHandler {

    /// Create the `products` table in the database if it doesn't yet exist
    pub fn create_table(&self) -> Result<usize, Error> {
        self.conn.execute(CREATE_PRODUCTS_TABLE)
            .map_err(|_| Error::db("cannot create the products table"))
    }

    /// Read a product from the database for the given UUID
    pub fn get_product_by_id(&self, id: &String) -> Result<Product, Error> {
        match self.conn.query(SELECT_PRODUCT_BY_ID, &[id]) {
            Ok(rows) => {
                if rows.is_empty() {
                    Err(Error::logic("multiple products with same id."))
                } else if rows.len() > 1 {
                    Err(Error::logic("no product with id found"))
                } else {
                    let row = rows.get(0);
                    Ok(
                        Product {
                            id: row.get(0),
                            name: row.get(1),
                            price: row.get(2),
                            description: row.get(3),
                            available: row.get(4)
                        }
                    )
                }
            },
            Err(_) => Err(Error::db("could not select product by id."))
        }
    }

    pub fn get_products(&self) -> Result<Vec<Product>, Error> {
        let rows = match self.conn.query(SELECT_PRODUCTS, &[]) {
            Ok(result) => result,
            Err(err) => return Err(
                Error::db(&format!("could not select all products: {}", err))
            )
        };

        let mut response: Vec<Product> = Vec::new();
        for row in rows.iter() {
            response.push(Product {
                id: row.get(0),
                name: row.get(1),
                price: row.get(2),
                description: row.get(3),
                available: row.get(4)
            });
        }
        Ok(response)
    }

    // Insert a product for a given UUID
    pub fn insert_product(&self, product: &Product) -> Result<Vec<Product>, Error> {
        if let Err(err) = self.conn.execute(
            INSERT_PRODUCT,
            &[
                &product.id,
                &product.name,
                &product.price,
                &product.description,
                &product.available
            ]) {
            return Err(Error::db(&format!("could not insert product: {:?}\n{}", product, err)));
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