#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;
extern crate argon2;

pub mod schema;
mod routes;
mod cust_error;
mod models;

#[database("pg_db")]
struct Database(rocket_contrib::databases::diesel::PgConnection);

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite()
        .attach(Database::fairing())
        .mount("/api", routes![index])
        .launch();
}
