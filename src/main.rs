use db::Db;
use dotenvy::dotenv;
use rocket::http::Status;
use rocket::Request;
use rocket_db_pools::Database;
use rocket_dyn_templates::Template;

#[macro_use]
extern crate rocket;

mod adapters;
mod db;
mod main_example;
mod middler;
mod posts;

use adapters::rabbitmq_adapter::RabbitMqAdapter;
use db::DbSqlx;
use middler::RemoveServerHeader;
use posts::create;
use posts::create2;
use posts::find;
use posts::find_all;
use rocket_async_compression::Compression;
use rocket_db_pools::Config;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
async fn rocket() -> _ {
    dotenv().expect(".env file not found");
    let figment = rocket::Config::figment().merge((
        "databases.sqlx",
        Config {
            max_connections: 20,
            connect_timeout: 1,
            idle_timeout: None,
            min_connections: Some(10),
            url: std::env::var("DATABASE_URL").expect("DATABASE_URL"),
        },
    ));

    let rabbitmq = RabbitMqAdapter::new().await;
    let _queue_stream = rabbitmq.create_stream("stream", 1).await;
    let _handler = rabbitmq.consumer("stream", None).await;

    rocket::build()
        .configure(figment)
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
