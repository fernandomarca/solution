use db::Db;
use dotenvy::dotenv;
use rocket::http::Status;
use rocket::Request;
use rocket_db_pools::Database;
use rocket_dyn_templates::Template;

#[macro_use]
extern crate rocket;

mod db;
mod main_example;
mod middler;
mod posts;

use db::DbSqlx;
use middler::RemoveServerHeader;
use posts::create;
use posts::create2;
use posts::find;
use posts::find_all;
use rocket_async_compression::Compression;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    dotenv().expect(".env file not found");

    rocket::build()
        .attach(Db::init())
        .attach(DbSqlx::init())
        .attach(RemoveServerHeader)
        .attach(Template::fairing())
        .attach(Compression::fairing())
        .register("/", catchers![internal_error, not_found, default])
        .mount("/", routes![index, create, create2, find, find_all])
}

#[catch(500)]
pub fn internal_error() -> &'static str {
    "internal server error"
}

#[catch(404)]
pub fn not_found(req: &Request) -> String {
    format!("I couldn't find '{}'. Try something else?", req.uri())
}

#[catch(default)]
pub fn default(status: Status, req: &Request) -> String {
    format!("{} ({})", status, req.uri())
}
