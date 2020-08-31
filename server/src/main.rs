#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
extern crate serde;
extern crate argon2;

use rocket_contrib::databases;
#[allow(unused_imports)] use diesel_migrations::embed_migrations;

pub mod schema;
mod routes;
mod cust_error;
mod models;

#[database("pg_db")]
pub struct Database(databases::diesel::PgConnection);

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    dotenv::dotenv().ok();

    #[cfg(not(debug_assertions))]
    embed_migrations!();

    rocket::ignite()
        .attach(Database::fairing())
        .mount("/api", routes![
            routes::users::post,
            routes::sessions::post,
            routes::sessions::delete,
        ])
        .launch();
}
