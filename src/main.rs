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

use db::Database;

mod db;
mod error;
mod endpoint;

static DB_URL: &'static str = "postgres://postgres@172.11.0.3";

fn init_db() -> Database {
    let database = db::Database::init(DB_URL);
    database.handler()
        .unwrap()
        .create()
        .unwrap();
    database
}

fn main() {
    println!("rustql!");

    rocket::ignite()
        .manage(init_db())
        .mount("/graphql", routes![endpoint::graphql_handler])
        .launch();
}
