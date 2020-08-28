#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use homrs_lib::initialize_db;
mod routes;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    let _ = initialize_db();
    rocket::ignite().mount("/api", routes![index]).launch();
}
