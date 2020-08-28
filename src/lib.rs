mod error;
pub mod db;
pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use db::PgPool;

pub fn initialize_db() {

}
