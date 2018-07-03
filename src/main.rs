#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

extern crate uuid;

#[macro_use] extern crate juniper;

mod schema;

extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;

extern crate rocket;

mod db;
mod error;
mod endpoint;

/// Postgres database URL
static DB_URL: &'static str = "postgres://postgres@172.11.0.3";

fn main() {
    println!("rustql!");

    // initialize the database and creates if not available a database instance
    let database = db::Database::init(DB_URL);
    if let Err(err) = database.handler().unwrap().create_table() {
        println!("Error creating `products` table: {}", err);
    }

    // mount the rocket endpoint with the database instance as state
    rocket::ignite()
        .manage(database)
        .mount("/graphql", routes![endpoint::graphql_handler])
        .launch();
}
