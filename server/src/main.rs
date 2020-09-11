#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
extern crate argon2;
extern crate serde;

#[allow(unused_imports)]
use diesel_migrations::embed_migrations;
use rocket_contrib::databases;

mod cust_error;
mod models;
mod routes;
pub mod schema;

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
        .mount(
            "/api",
            routes![
                routes::users::post,
                routes::users::put,
                routes::users::delete,
                routes::users::change_password,
                routes::sessions::post,
                routes::sessions::status,
                routes::sessions::delete,
                routes::recipes::post,
                routes::ingredients::post,
            ],
        )
        .launch();
}
