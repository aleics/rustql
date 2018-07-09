#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

extern crate uuid;

#[macro_use] extern crate juniper;

#[macro_use] extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;

extern crate rocket;

use db::Database;
use rocket::Rocket;
use std::env;

mod db;
mod error;
mod endpoint;
mod schema;
mod graphql;
mod models;

fn rocket(database: Database) -> Rocket {
    rocket::ignite()
        .manage(database)
        .mount("/api", routes![endpoint::graphql_handler])
}

fn main() {
    println!("rustql!");

    let db_url = env::var("DB_URL").expect("DB_URL must be set");

    // initialize the database and creates if not available a database instance
    let database = db::Database::init(&db_url);
    if let Err(err) = database.handler().unwrap().create_table() {
        println!("Error creating `products` table: {}", err);
    }

    // mount the rocket endpoint with the database instance as state
    rocket(database).launch();
}
